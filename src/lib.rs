use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64::consts::PI;
use js_sys::Math;

#[derive(Clone)]
struct Vector {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, PartialEq)]
enum DisruptionType {
    None,
    Disabled,
    Scrambled,
    Uncontrollable,
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
        
        // Apply friction to velocity
        self.velocity.x *= 0.99;
        self.velocity.y *= 0.99;
        
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
        let start_position = Vector {
            x: ship.position.x + 20.0 * ship.rotation.sin(),
            y: ship.position.y - 20.0 * ship.rotation.cos(),
        };
        Bullet {
            position: start_position,
            velocity: Vector {
                x: speed * ship.rotation.sin(),
                y: -speed * ship.rotation.cos(),
            },
            lifetime: 60,
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

struct ShockWave {
    position: Vector,
    radius: f64,
    max_radius: f64,
    alpha: f64,
    lifetime: i32,
}

impl ShockWave {
    fn new(x: f64, y: f64) -> Self {
        ShockWave {
            position: Vector { x, y },
            radius: 0.0,
            max_radius: 150.0,
            alpha: 1.0,
            lifetime: 60,
        }
    }

    fn update(&mut self) -> bool {
        self.radius += 3.0;
        self.alpha = (self.lifetime as f64) / 60.0;
        self.lifetime -= 1;
        self.lifetime > 0 && self.radius < self.max_radius
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        ctx.set_global_alpha(self.alpha);
        ctx.set_stroke_style_str("#00ffff");
        ctx.set_line_width(3.0);
        
        // Draw multiple expanding rings
        for i in 0..3 {
            let ring_radius = self.radius - (i as f64 * 15.0);
            if ring_radius > 0.0 {
                ctx.begin_path();
                ctx.arc(self.position.x, self.position.y, ring_radius, 0.0, 2.0 * PI).unwrap();
                ctx.stroke();
            }
        }
        
        ctx.restore();
    }
}

struct Asteroid {
    position: Vector,
    velocity: Vector,
    size: f64,
    is_special: bool,
    pulse_time: f64,
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
            is_special: false,
            pulse_time: 0.0,
        }
    }

    fn new_special(x: f64, y: f64, size: f64) -> Self {
        let mut asteroid = Self::new(x, y, size);
        asteroid.is_special = true;
        asteroid
    }

    fn update(&mut self, speed_multiplier: f64) {
        self.position.x += self.velocity.x * speed_multiplier;
        self.position.y += self.velocity.y * speed_multiplier;
        
        if self.is_special {
            self.pulse_time += 0.1;
        }
        
        // Wrap around screen
        if self.position.x > 800.0 { self.position.x = 0.0; }
        if self.position.x < 0.0 { self.position.x = 800.0; }
        if self.position.y > 600.0 { self.position.y = 0.0; }
        if self.position.y < 0.0 { self.position.y = 600.0; }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        
        if self.is_special {
            // Special asteroid visual effects
            let pulse = (self.pulse_time.sin() * 0.5 + 0.5) * 0.3 + 0.7;
            ctx.set_global_alpha(pulse);
            ctx.set_stroke_style_str("#ff0080");
            ctx.set_line_width(2.0);
            
            // Draw electric arcs around the asteroid
            for i in 0..4 {
                let angle = (i as f64) * PI * 0.5 + self.pulse_time;
                let arc_x = self.position.x + (self.size + 10.0) * angle.cos();
                let arc_y = self.position.y + (self.size + 10.0) * angle.sin();
                ctx.begin_path();
                ctx.move_to(self.position.x, self.position.y);
                ctx.line_to(arc_x, arc_y);
                ctx.stroke();
            }
        } else {
            ctx.set_stroke_style_str("white");
            ctx.set_line_width(1.0);
        }
        
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
        
        ctx.restore();
    }

    fn split(&self) -> Vec<Asteroid> {
        if self.size < 15.0 || self.is_special {
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
    shock_waves: Vec<ShockWave>,
    game_over: bool,
    score: i32,
    lives: i32,
    level: i32,
    respawn_timer: i32,
    level_transition_timer: i32,
    disruption_type: DisruptionType,
    disruption_timer: i32,
    uncontrollable_force: Vector,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<Game, JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut game = Game {
            ship: Ship::new(400.0, 300.0),
            ctx,
            asteroids: Vec::new(),
            bullets: Vec::new(),
            shock_waves: Vec::new(),
            game_over: false,
            score: 0,
            lives: 3,
            level: 1,
            respawn_timer: 0,
            level_transition_timer: 0,
            disruption_type: DisruptionType::None,
            disruption_timer: 0,
            uncontrollable_force: Vector { x: 0.0, y: 0.0 },
        };

        game.spawn_level_asteroids();
        Ok(game)
    }

    fn spawn_level_asteroids(&mut self) {
        self.asteroids.clear();
        let asteroid_count = 4 + (self.level - 1) / 4; // Add extra asteroid every 4 levels
        
        for _ in 0..asteroid_count {
            let x = Math::random() * 800.0;
            let y = Math::random() * 600.0;
            
            // 1 in 50 chance for special asteroid (increased for testing)
            if Math::random() < 1.0 / 50.0 {
                self.asteroids.push(Asteroid::new_special(x, y, 40.0));
            } else {
                self.asteroids.push(Asteroid::new(x, y, 40.0));
            }
        }
    }

    fn get_speed_multiplier(&self) -> f64 {
        let base_speed = 1.0;
        let level_factor = ((self.level - 1) % 3) as f64 * 0.3; // Speed increases within each 3-level cycle
        base_speed + level_factor
    }

    fn get_score_multiplier(&self) -> i32 {
        1 + (self.level - 1) / 3 // Small score bonus every 3 levels
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Handle level transition
        if self.level_transition_timer > 0 {
            self.level_transition_timer -= 1;
            if self.level_transition_timer == 0 {
                self.spawn_level_asteroids();
            }
            return;
        }

        // Handle disruption timer
        if self.disruption_timer > 0 {
            self.disruption_timer -= 1;
            if self.disruption_timer == 0 {
                self.disruption_type = DisruptionType::None;
                self.uncontrollable_force = Vector { x: 0.0, y: 0.0 };
            }
        }

        // Apply uncontrollable force if active
        if self.disruption_type == DisruptionType::Uncontrollable {
            self.ship.velocity.x += self.uncontrollable_force.x;
            self.ship.velocity.y += self.uncontrollable_force.y;
        }

        // Decrease respawn timer if active
        if self.respawn_timer > 0 {
            self.respawn_timer -= 1;
        }

        self.ship.update();
        
        // Check ship-asteroid collisions only if not in respawn period
        if self.respawn_timer == 0 {
            for asteroid in &self.asteroids {
                let dx = self.ship.position.x - asteroid.position.x;
                let dy = self.ship.position.y - asteroid.position.y;
                if (dx * dx + dy * dy).sqrt() < asteroid.size + 10.0 {
                    self.lives -= 1;
                    play_sound("explosion-sound");
                    
                    if self.lives <= 0 {
                        self.game_over = true;
                        return;
                    } else {
                        // Respawn ship in center with temporary invulnerability
                        self.ship = Ship::new(400.0, 300.0);
                        self.respawn_timer = 180; // 3 seconds at 60 FPS
                        return;
                    }
                }
            }
        }
        
        // Update bullets and remove dead ones
        self.bullets.retain_mut(|bullet| bullet.update());
        
        // Update asteroids with speed multiplier
        let speed_multiplier = self.get_speed_multiplier();
        for asteroid in &mut self.asteroids {
            asteroid.update(speed_multiplier);
        }

        // Update shock waves
        self.shock_waves.retain_mut(|wave| wave.update());

        // Check bullet collisions
        let mut new_asteroids = Vec::new();
        let mut bullets_to_remove = Vec::new();
        let mut asteroids_to_remove = Vec::new();
        
        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            for (asteroid_idx, asteroid) in self.asteroids.iter().enumerate() {
                let dx = bullet.position.x - asteroid.position.x;
                let dy = bullet.position.y - asteroid.position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < asteroid.size {
                    bullets_to_remove.push(bullet_idx);
                    asteroids_to_remove.push(asteroid_idx);
                    
                    let score_multiplier = self.get_score_multiplier();
                    
                    if asteroid.is_special {
                        // Special asteroid hit
                        self.score += 100 * score_multiplier;
                        self.shock_waves.push(ShockWave::new(asteroid.position.x, asteroid.position.y));
                        play_sound("special-explosion-sound");
                        
                        // Apply random disruption
                        self.disruption_timer = 120; // 2 seconds at 60 FPS
                        let random_disruption = Math::random();
                        if random_disruption < 0.33 {
                            self.disruption_type = DisruptionType::Disabled;
                        } else if random_disruption < 0.66 {
                            self.disruption_type = DisruptionType::Scrambled;
                        } else {
                            self.disruption_type = DisruptionType::Uncontrollable;
                            // Random uncontrollable force
                            let angle = Math::random() * 2.0 * PI;
                            self.uncontrollable_force = Vector {
                                x: 0.2 * angle.sin(),
                                y: -0.2 * angle.cos(),
                            };
                        }
                    } else {
                        // Regular asteroid scoring
                        self.score += if asteroid.size >= 40.0 {
                            20 * score_multiplier  // Large asteroid
                        } else if asteroid.size >= 20.0 {
                            50 * score_multiplier  // Medium asteroid
                        } else {
                            100 * score_multiplier  // Small asteroid
                        };
                        
                        new_asteroids.extend(asteroid.split());
                    }
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

        // Check if level is complete
        if self.asteroids.is_empty() && self.level_transition_timer == 0 {
            self.level += 1;
            self.level_transition_timer = 300; // 5 seconds at 60 FPS
            play_sound("level-complete-sound");
        }
    }

    pub fn render(&self) {
        self.ctx.clear_rect(0.0, 0.0, 800.0, 600.0);
        self.ctx.set_stroke_style_str("white");
        self.ctx.set_fill_style_str("white");
        
        // Draw score and level at top left
        self.ctx.set_font("24px Arial");
        self.ctx.set_text_align("left");
        self.ctx.fill_text(&format!("Score: {}", self.score), 20.0, 40.0).unwrap();
        self.ctx.fill_text(&format!("Level: {}", self.level), 20.0, 70.0).unwrap();
        
        // Draw lives at top right
        self.ctx.set_text_align("right");
        self.ctx.fill_text(&format!("Lives: {}", self.lives), 780.0, 40.0).unwrap();

        // Draw disruption status
        if self.disruption_timer > 0 {
            self.ctx.set_text_align("center");
            self.ctx.set_fill_style_str("#ff0080");
            let disruption_text = match self.disruption_type {
                DisruptionType::Disabled => "SYSTEMS DISABLED",
                DisruptionType::Scrambled => "CONTROLS SCRAMBLED",
                DisruptionType::Uncontrollable => "SHIP UNSTABLE",
                DisruptionType::None => "",
            };
            self.ctx.fill_text(disruption_text, 400.0, 100.0).unwrap();
            self.ctx.set_fill_style_str("white");
        }
        
        // Draw level transition countdown
        if self.level_transition_timer > 0 {
            self.ctx.set_font("48px Arial");
            self.ctx.set_text_align("center");
            self.ctx.fill_text(&format!("LEVEL {}", self.level), 400.0, 280.0).unwrap();
            self.ctx.set_font("24px Arial");
            let countdown = (self.level_transition_timer / 60) + 1;
            self.ctx.fill_text(&format!("Starting in {}...", countdown), 400.0, 320.0).unwrap();
            self.ctx.fill_text("Get Ready!", 400.0, 350.0).unwrap();
            return;
        }
        
        if !self.game_over {
            // Make ship blink during respawn period
            if self.respawn_timer == 0 || self.respawn_timer % 20 > 10 {
                self.ship.draw(&self.ctx);
            }
        }
        
        for bullet in &self.bullets {
            bullet.draw(&self.ctx);
        }
        
        for asteroid in &self.asteroids {
            asteroid.draw(&self.ctx);
        }

        for shock_wave in &self.shock_waves {
            shock_wave.draw(&self.ctx);
        }

        if self.game_over {
            self.ctx.set_font("48px Arial");
            self.ctx.set_text_align("center");
            self.ctx.fill_text("GAME OVER", 400.0, 280.0).unwrap();
            self.ctx.set_font("24px Arial");
            self.ctx.fill_text(&format!("Final Score: {}", self.score), 400.0, 320.0).unwrap();
            self.ctx.fill_text(&format!("Level Reached: {}", self.level), 400.0, 350.0).unwrap();
            self.ctx.fill_text("Press R or click New Game to restart", 400.0, 380.0).unwrap();
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        if self.disruption_type == DisruptionType::Disabled {
            return;
        }
        
        let actual_angle = if self.disruption_type == DisruptionType::Scrambled {
            -angle // Reverse controls
        } else {
            angle
        };
        
        self.ship.rotation += actual_angle;
    }

    pub fn thrust(&mut self) {
        if self.disruption_type == DisruptionType::Disabled {
            return;
        }
        
        let thrust = 0.1;
        let (thrust_x, thrust_y) = if self.disruption_type == DisruptionType::Scrambled {
            // Scrambled: thrust goes sideways
            (thrust * self.ship.rotation.cos(), thrust * self.ship.rotation.sin())
        } else {
            // Normal thrust
            (thrust * self.ship.rotation.sin(), -thrust * self.ship.rotation.cos())
        };
        
        self.ship.velocity.x += thrust_x;
        self.ship.velocity.y += thrust_y;
        play_sound("thrust-sound");
    }

    pub fn shoot(&mut self) {
        if self.disruption_type == DisruptionType::Disabled {
            return;
        }
        
        self.bullets.push(Bullet::new(&self.ship));
        play_sound("shoot-sound");
    }

    pub fn reset(&mut self) {
        self.game_over = false;
        self.ship = Ship::new(400.0, 300.0);
        self.bullets.clear();
        self.asteroids.clear();
        self.shock_waves.clear();
        self.score = 0;
        self.lives = 3;
        self.level = 1;
        self.respawn_timer = 0;
        self.level_transition_timer = 0;
        self.disruption_type = DisruptionType::None;
        self.disruption_timer = 0;
        self.uncontrollable_force = Vector { x: 0.0, y: 0.0 };
        
        self.spawn_level_asteroids();
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }
}

fn play_sound(sound_id: &str) {
    let _window = match web_sys::window() {
        Some(win) => win,
        None => return,
    };

    // Create AudioContext
    let audio_context = match web_sys::AudioContext::new() {
        Ok(ctx) => ctx,
        Err(_) => return,
    };

    // Create oscillator
    let oscillator = match audio_context.create_oscillator() {
        Ok(osc) => osc,
        Err(_) => return,
    };

    // Create gain node for volume control
    let gain = match audio_context.create_gain() {
        Ok(g) => g,
        Err(_) => return,
    };

    // Configure sound based on type
    match sound_id {
        "shoot-sound" => {
            oscillator.set_type(web_sys::OscillatorType::Square);
            oscillator.frequency().set_value(440.0); // A4 note
            gain.gain().set_value(0.1); // Lower volume
            let _ = gain.gain().linear_ramp_to_value_at_time(0.0, audio_context.current_time() + 0.1);
        }
        "explosion-sound" => {
            oscillator.set_type(web_sys::OscillatorType::Sawtooth);
            oscillator.frequency().set_value(100.0); // Low frequency
            gain.gain().set_value(0.3);
            let _ = gain.gain().linear_ramp_to_value_at_time(0.0, audio_context.current_time() + 0.3);
        }
        "thrust-sound" => {
            oscillator.set_type(web_sys::OscillatorType::Sawtooth);
            oscillator.frequency().set_value(200.0); // Low whoosh sound
            gain.gain().set_value(0.08); // Very quiet
            let _ = oscillator.frequency().linear_ramp_to_value_at_time(
                100.0,
                audio_context.current_time() + 0.1
            ); // Frequency drop for whoosh effect
            let _ = gain.gain().linear_ramp_to_value_at_time(0.0, audio_context.current_time() + 0.1);
        }
        "special-explosion-sound" => {
            oscillator.set_type(web_sys::OscillatorType::Sawtooth);
            oscillator.frequency().set_value(80.0); // Very low, powerful sound
            gain.gain().set_value(0.4);
            let _ = oscillator.frequency().linear_ramp_to_value_at_time(
                200.0,
                audio_context.current_time() + 0.2
            ); // Frequency sweep up
            let _ = gain.gain().linear_ramp_to_value_at_time(0.0, audio_context.current_time() + 0.5);
        }
        "level-complete-sound" => {
            oscillator.set_type(web_sys::OscillatorType::Sine);
            oscillator.frequency().set_value(523.25); // C5 note
            gain.gain().set_value(0.2);
            let _ = oscillator.frequency().linear_ramp_to_value_at_time(
                783.99, // G5 note
                audio_context.current_time() + 0.3
            );
            let _ = gain.gain().linear_ramp_to_value_at_time(0.0, audio_context.current_time() + 0.4);
        }
        _ => return,
    }

    // Connect nodes
    let _ = oscillator.connect_with_audio_node(&gain);
    let _ = gain.connect_with_audio_node(&audio_context.destination());

    // Start and stop the sound
    let _ = oscillator.start();
    let _ = oscillator.stop_with_when(audio_context.current_time() + 0.5);
}