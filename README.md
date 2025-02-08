# WASM Asteroids

A WebAssembly implementation of the classic Asteroids game using Rust.

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A local web server (Python's `http.server`, Node's `http-server`, or similar)

## Building the Game

```bash
wasm-pack build --target web
```

## Running the Game

```bash
python3 -m http.server 8080
```

## How to Play

Controls:
- Left Arrow: Rotate left
- Right Arrow: Rotate right
- Up Arrow: Thrust forward
- Spacebar: Shoot
- R key or "New Game" button: Start a new game

Game Rules:
- Shoot the asteroids to break them into smaller pieces
- Large asteroids break into two smaller ones
- The smallest asteroids disappear when shot
- Avoid letting asteroids hit your ship
- Press R or click "New Game" to start over

## Sound Credits
- Laser sound effect: [credit source]
- Explosion sound effect: [credit source]