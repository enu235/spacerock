# 🚀 WASM Asteroids - Enhanced Edition 🌌

A WebAssembly implementation of the classic Asteroids game using Rust, featuring progressive levels, special asteroids with unique effects, and immersive gameplay mechanics.

## ✨ New Features

### 🎯 Progressive Level System
- **Dynamic Difficulty:** Asteroid speed increases with each level
- **Level Cycles:** Every 3 levels, speed resets and increases again
- **Bonus Asteroids:** Every 4th level adds an extra asteroid to the field
- **5-Second Countdown:** Prepare between levels with dramatic transitions
- **Score Multipliers:** Higher levels provide bonus points

### ⚡ Special Asteroids
- **Rare Spawning:** 1 in 50 chance for special asteroids (increased for testing)
- **Visual Effects:** Pink electric arcs and pulsing animations
- **No Splitting:** Special asteroids are completely destroyed when hit
- **High Rewards:** 100+ points (scales with level)
- **Shock Wave Effects:** Spectacular expanding ring animations

### 🛸 Ship Disruption System
When special asteroids are destroyed, one of three random disruptions occurs for 2 seconds:

1. **Systems Disabled:** All controls are temporarily disabled
2. **Controls Scrambled:** Movement controls are reversed/altered
3. **Ship Instability:** Uncontrollable drift forces applied to the ship

### 🎵 Enhanced Audio
- **Dynamic Sound Effects:** Improved thrust, shooting, and explosion sounds
- **Special Effects Audio:** Unique sound for special asteroid destruction
- **Level Progression:** Musical chimes for level completion

### 🎨 Visual Enhancements
- **Modern UI:** Gradient backgrounds and glowing effects
- **Real-time Status:** Disruption indicators and level information
- **Responsive Design:** Mobile-friendly interface
- **Particle Effects:** Shock waves and electric arcs

## 📋 Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A local web server (Python's `http.server`, Node's `http-server`, or similar)

## 🔧 Building the Game

```bash
# Clone the repository
git clone <your-repo-url>
cd astroids

# Build the WebAssembly package
wasm-pack build --target web

# Start a local server (choose one)
python3 -m http.server 8080
# OR
npx http-server -p 8080
# OR
php -S localhost:8080
```

Then open your browser to `http://localhost:8080`

## 🎮 How to Play

### Controls
- **Left Arrow (←):** Rotate left
- **Right Arrow (→):** Rotate right  
- **Up Arrow (↑):** Thrust forward
- **Spacebar:** Shoot
- **R key or "New Game" button:** Start a new game

### Scoring System
- **Large Asteroid:** 20 points × level multiplier
- **Medium Asteroid:** 50 points × level multiplier  
- **Small Asteroid:** 100 points × level multiplier
- **Special Asteroid:** 100+ points × level multiplier

### Level Progression
- Clear all asteroids to advance to the next level
- Speed increases every level within 3-level cycles
- Every 4th level adds an additional asteroid
- Score multipliers increase every 3 levels

### Special Mechanics
- **Watch for Special Asteroids:** Pink asteroids with electric effects
- **Prepare for Disruption:** Special asteroids cause temporary control issues
- **Strategic Planning:** Use level countdown time to position yourself
- **Risk vs Reward:** Special asteroids give high points but create challenges

## 🛠️ Technical Details

### Technologies Used
- **Rust** - Core game logic and performance
- **WebAssembly (WASM)** - Browser compatibility and speed
- **Web Canvas API** - 2D rendering and graphics
- **Web Audio API** - Dynamic sound generation
- **Modern CSS** - Responsive UI and visual effects

### Architecture
- **Game Loop:** 60 FPS rendering with `requestAnimationFrame`
- **Entity System:** Modular ship, asteroid, bullet, and effect objects
- **State Management:** Comprehensive game state with level progression
- **Effect System:** Particle effects and visual feedback
- **Audio Engine:** Procedural sound generation

## 🎯 Game Strategy Tips

1. **Master the Physics:** Use momentum and wrapping to your advantage
2. **Target Priority:** Focus on large asteroids first to prevent splitting
3. **Special Asteroid Timing:** Plan your approach to special asteroids carefully
4. **Disruption Survival:** Stay calm during control disruptions
5. **Level Preparation:** Use countdown time to position optimally
6. **Score Optimization:** Higher levels give better score multipliers

## 🚀 Future Enhancements

- **Power-ups:** Temporary abilities and weapons
- **Boss Asteroids:** Unique challenging enemies
- **Multiplayer Mode:** Cooperative and competitive gameplay
- **Achievement System:** Unlock rewards and challenges
- **Custom Ship Designs:** Visual customization options

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs and issues
- Suggest new features
- Submit pull requests
- Improve documentation

## 📄 License

This project is open source. Please check the license file for details.

## 🎮 Credits

- **Original Asteroids:** Inspired by Atari's classic 1979 arcade game
- **Enhanced Features:** Modern gameplay mechanics and visual effects
- **Audio Design:** Procedural sound generation using Web Audio API
- **Visual Effects:** Custom particle systems and animations

---

**Made with 💖 and Rust** 

Enjoy the enhanced Asteroids experience! 🌌✨