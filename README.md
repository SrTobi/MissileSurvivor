# Missile Survivor

[![Deploy to GitHub Pages](https://img.shields.io/github/actions/workflow/status/SrTobi/MissileSurvivor/deploy-gh-pages.yml
)](https://github.com/SrTobi/MissileSurvivor/actions/workflows/deploy-gh-pages.yml)

[Play the game online](https://srtobi.github.io/MissileSurvivor/)

A missile defense game built with Rust and Macroquad where you defend your bunkers from incoming missiles.


## Game Description

In Missile Survivor, you control a set of bunkers at the bottom of the screen. Enemy missiles rain down from the top of the screen, targeting your bunkers. Your goal is to intercept these missiles by launching your own missiles from the bunkers.

As you destroy enemy missiles, you gain experience points based on how high you intercept them. Collect blue stars to level up and improve your skills, making your defenses more effective.

The game gets progressively harder over time, with enemy missiles becoming faster and more frequent. How long can you survive?

## Features

- Fast-paced missile defense gameplay
- Skill progression system with multiple upgradeable abilities
- Chain reaction explosions
- Progressive difficulty
- WebAssembly support for playing in browsers

## Controls

- **Mouse Click**: Fire a missile from the nearest active bunker to the clicked location
- **Mouse Hover + Click**: Select skills when leveling up
- **ESC**: Exit the game (not available in web version)
- **Any Key**: Restart after game over

## Building and Running

### Prerequisites

- Rust and Cargo (https://rustup.rs/)
- For WASM builds: wasm-bindgen-cli (`cargo install wasm-bindgen-cli`)

### Native Build

```bash
# Debug build
cargo run

# Release build
cargo run --release
```

### Web Build (WebAssembly)

```bash
# Debug build
./build-wasm-client.sh

# Release build
./build-wasm-client.sh --release
```

After building for web, the output will be in the `dist/debug` or `dist/release` directory. You can serve these files with any HTTP server, for example:

```bash
# Using Python's built-in HTTP server
cd dist/release
python -m http.server
```

Then open your browser to http://localhost:8000

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
