# 🚀 WASM SpaceRock - Enhanced Edition 🌌

A WebAssembly implementation of the classic SpaceRock game using Rust, featuring progressive levels, special space rocks with unique effects, and immersive gameplay mechanics.

## ⚡ Quick Start

### 🎮 One-Click Setup & Play

For the fastest way to get SpaceRock running, use our automated setup scripts:

#### **Unix/Linux/macOS Users:**
```bash
# Download and run the setup script
curl -sSL https://raw.githubusercontent.com/enu235/spacerock/main/quick-start.sh | bash
```

Or download the script first:
```bash
wget https://raw.githubusercontent.com/enu235/spacerock/main/quick-start.sh
chmod +x quick-start.sh
./quick-start.sh
```

#### **Windows Users:**
1. Download [`quick-start.bat`](https://raw.githubusercontent.com/enu235/spacerock/main/quick-start.bat)
2. Double-click the file or run from Command Prompt:
```cmd
quick-start.bat
```

### 🔧 What the Scripts Do:
- ✅ Check for Rust installation (install instructions if missing)
- ✅ Check for wasm-pack (auto-install if possible)
- ✅ Check for Git (install instructions if missing)  
- ✅ Check for web server capability (Python 3 or PHP recommended, Node.js optional)
- ✅ Clone the repository automatically
- ✅ Build the WebAssembly package
- ✅ Start local web server
- ✅ Open the game in your browser

**That's it! The game will be running and ready to play!** 🎯

### 📋 What You Actually Need:
- **Required**: Rust + wasm-pack (for building WebAssembly)
- **Required**: Git (for downloading the code)
- **Required**: Web server (Python 3, PHP, or Node.js - any one will work)
- **Not Required**: Node.js specifically (only if you want to use it as a web server option)

---

## ✨ New Features

### 🎯 Progressive Level System
- **Dynamic Difficulty:** Space rock speed increases with each level
- **Level Cycles:** Every 3 levels, speed resets and increases again
- **Bonus Space Rocks:** Every 4th level adds an extra space rock to the field
- **5-Second Countdown:** Prepare between levels with dramatic transitions
- **Score Multipliers:** Higher levels provide bonus points

### ⚡ Special Space Rocks
- **Rare Spawning:** Random chance between 1 in 100 to 1 in 300 for special space rocks
- **Visual Effects:** Pink electric arcs and pulsing animations
- **No Splitting:** Special space rocks are completely destroyed when hit
- **High Rewards:** 100+ points (scales with level)
- **Shock Wave Effects:** Spectacular expanding ring animations

### 🛸 Ship Disruption System
When special space rocks are destroyed, one of three random disruptions occurs for 2 seconds:

1. **Systems Disabled:** All controls are temporarily disabled
2. **Controls Scrambled:** Movement controls are reversed/altered
3. **Ship Instability:** Uncontrollable drift forces applied to the ship

### 🎵 Enhanced Audio
- **Dynamic Sound Effects:** Improved thrust, shooting, and explosion sounds
- **Special Effects Audio:** Unique sound for special space rock destruction
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
git clone https://github.com/enu235/spacerock.git
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
- **Large Space Rock:** 20 points × level multiplier
- **Medium Space Rock:** 50 points × level multiplier  
- **Small Space Rock:** 100 points × level multiplier
- **Special Space Rock:** 100+ points × level multiplier

### Level Progression
- Clear all space rocks to advance to the next level
- Speed increases every level within 3-level cycles
- Every 4th level adds an additional space rock
- Score multipliers increase every 3 levels

### Special Mechanics
- **Watch for Special Space Rocks:** Pink space rocks with electric effects
- **Prepare for Disruption:** Special space rocks cause temporary control issues
- **Strategic Planning:** Use level countdown time to position yourself
- **Risk vs Reward:** Special space rocks give high points but create challenges

## 🛠️ Technical Details

### Technologies Used
- **Rust** - Core game logic and performance
- **WebAssembly (WASM)** - Browser compatibility and speed
- **Web Canvas API** - 2D rendering and graphics
- **Web Audio API** - Dynamic sound generation
- **Modern CSS** - Responsive UI and visual effects

### Architecture
- **Game Loop:** 60 FPS rendering with `requestAnimationFrame`
- **Entity System:** Modular ship, space rock, bullet, and effect objects
- **State Management:** Comprehensive game state with level progression
- **Effect System:** Particle effects and visual feedback
- **Audio Engine:** Procedural sound generation

## 🎯 Game Strategy Tips

1. **Master the Physics:** Use momentum and wrapping to your advantage
2. **Target Priority:** Focus on large space rocks first to prevent splitting
3. **Special Space Rock Timing:** Plan your approach to special space rocks carefully
4. **Disruption Survival:** Stay calm during control disruptions
5. **Level Preparation:** Use countdown time to position optimally
6. **Score Optimization:** Higher levels give better score multipliers

## 🚀 Future Enhancements

- **Power-ups:** Temporary abilities and weapons
- **Boss Space Rocks:** Unique challenging enemies
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

- **Original SpaceRock:** Inspired by Atari's classic 1979 arcade game Asteroids
- **Enhanced Features:** Modern gameplay mechanics and visual effects
- **Audio Design:** Procedural sound generation using Web Audio API
- **Visual Effects:** Custom particle systems and animations

---

**Made with 💖 and Rust** 

Enjoy the enhanced SpaceRock experience! 🌌✨