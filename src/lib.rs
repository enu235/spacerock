use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;
use js_sys::Math;

#[derive(Clone)]
struct Vector {
    x: f64,
    y: f64,
}

struct Ship {
    position: Vector,
    velocity: Vector,
    rotation: f64,
}

impl Ship {
    fn new(x: f64, y: f64) -> Ship {
        Ship {
            position: Vector { x, y },
            velocity: Vector { x: 0.0, y: 0.0 },
            rotation: 0.0,
        }
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        
        // Wrap around screen
        if self.position.x > 800.0 { self.position.x = 0.0; }
        if self.position.x < 0.0 { self.position.x = 800.0; }
        if self.position.y > 600.0 { self.position.y = 0.0; }
        if self.position.y < 0.0 { self.position.y = 600.0; }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.translate(self.position.x, self.position.y).unwrap();
        ctx.rotate(self.rotation).unwrap();
        
        ctx.begin_path();
        ctx.move_to(0.0, -20.0);
        ctx.line_to(10.0, 20.0);
        ctx.line_to(-10.0, 20.0);
        ctx.close_path();
        ctx.stroke();
        
        ctx.restore();
    }
}

struct Bullet {
    position: Vector,
    velocity: Vector,
    lifetime: i32,
}

impl Bullet {
    fn new(ship: &Ship) -> Self {
        let speed = 10.0;
        Bullet {
            position: ship.position.clone(),
            velocity: Vector {
                x: speed * ship.rotation.sin(),
                y: -speed * ship.rotation.cos(),
            },
            lifetime: 60, // frames the bullet lives for
        }
    }

    fn update(&mut self) -> bool {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        
        // Wrap around screen
        if self.position.x > 800.0 { self.position.x = 0.0; }
        if self.position.x < 0.0 { self.position.x = 800.0; }
        if self.position.y > 600.0 { self.position.y = 0.0; }
        if self.position.y < 0.0 { self.position.y = 600.0; }

        self.lifetime -= 1;
        self.lifetime > 0
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.position.x, self.position.y, 2.0, 0.0, 2.0 * PI).unwrap();
        ctx.fill();
    }
}

struct Asteroid {
    position: Vector,
    velocity: Vector,
    size: f64,
}

impl Asteroid {
    fn new(x: f64, y: f64, size: f64) -> Self {
        let speed = 2.0;
        let angle = Math::random() * 2.0 * PI;
        Asteroid {
            position: Vector { x, y },
            velocity: Vector {
                x: speed * angle.sin(),
                y: -speed * angle.cos(),
            },
            size,
        }
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        
        // Wrap around screen
        if self.position.x > 800.0 { self.position.x = 0.0; }
        if self.position.x < 0.0 { self.position.x = 800.0; }
        if self.position.y > 600.0 { self.position.y = 0.0; }
        if self.position.y < 0.0 { self.position.y = 600.0; }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        for i in 0..8 {
            let angle = (i as f64) * PI / 4.0;
            let radius = self.size * (0.8 + 0.4 * Math::random());
            let x = self.position.x + radius * angle.cos();
            let y = self.position.y + radius * angle.sin();
            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.close_path();
        ctx.stroke();
    }

    fn split(&self) -> Vec<Asteroid> {
        if self.size < 15.0 {
            return vec![];
        }
        
        vec![
            Asteroid::new(self.position.x, self.position.y, self.size / 2.0),
            Asteroid::new(self.position.x, self.position.y, self.size / 2.0),
        ]
    }
}

#[wasm_bindgen]
pub struct Game {
    ship: Ship,
    ctx: CanvasRenderingContext2d,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Game, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Create initial asteroids
        let mut asteroids = Vec::new();
        for _ in 0..4 {
            let x = Math::random() * 800.0;
            let y = Math::random() * 600.0;
            asteroids.push(Asteroid::new(x, y, 40.0));
        }

        Ok(Game {
            ship: Ship::new(400.0, 300.0),
            ctx,
            asteroids,
            bullets: Vec::new(),
        })
    }

    pub fn update(&mut self) {
        self.ship.update();
        
        // Update bullets and remove dead ones
        self.bullets.retain_mut(|bullet| bullet.update());
        
        // Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update();
        }

        // Check bullet collisions
        let mut new_asteroids = Vec::new();
        let mut bullets_to_remove = Vec::new();
        let mut asteroids_to_remove = Vec::new();
        
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            for (asteroid_idx, asteroid) in self.asteroids.iter().enumerate() {
                let dx = bullet.position.x - asteroid.position.x;
                let dy = bullet.position.y - asteroid.position.y;
                if (dx * dx + dy * dy).sqrt() < asteroid.size {
                    bullets_to_remove.push(bullet_idx);
                    asteroids_to_remove.push(asteroid_idx);
                    new_asteroids.extend(asteroid.split());
                    break;
                }
            }
        }

        // Remove asteroids in reverse order to maintain correct indices
        asteroids_to_remove.sort_unstable();
        asteroids_to_remove.reverse();
        for asteroid_idx in asteroids_to_remove {
            self.asteroids.remove(asteroid_idx);
        }

        // Remove bullets in reverse order to maintain correct indices
        bullets_to_remove.sort_unstable();
        bullets_to_remove.reverse();
        for bullet_idx in bullets_to_remove {
            self.bullets.remove(bullet_idx);
        }

        // Add new asteroids from splits
        self.asteroids.extend(new_asteroids);
    }

    pub fn render(&self) {
        self.ctx.clear_rect(0.0, 0.0, 800.0, 600.0);
        self.ctx.set_stroke_style(&JsValue::from_str("white"));
        self.ctx.set_fill_style(&JsValue::from_str("white"));
        
        self.ship.draw(&self.ctx);
        
        for bullet in &self.bullets {
            bullet.draw(&self.ctx);
        }
        
        for asteroid in &self.asteroids {
            asteroid.draw(&self.ctx);
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        self.ship.rotation += angle;
    }

    pub fn thrust(&mut self) {
        let thrust = 0.1;
        self.ship.velocity.x += thrust * self.ship.rotation.sin();
        self.ship.velocity.y -= thrust * self.ship.rotation.cos();
    }

    pub fn shoot(&mut self) {
        self.bullets.push(Bullet::new(&self.ship));
    }

    pub fn reset(&mut self) {
        self.ship = Ship::new(400.0, 300.0);
        self.bullets.clear();
        self.asteroids.clear();
        
        // Create initial asteroids
        for _ in 0..4 {
            let x = Math::random() * 800.0;
            let y = Math::random() * 600.0;
            self.asteroids.push(Asteroid::new(x, y, 40.0));
        }
    }
} 