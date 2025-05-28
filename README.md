# Rust Raylib Game Collection

A collection of simple games built with Rust and Raylib, featuring a borderless fullscreen window and smooth transitions between games.

## Games

### Pong
- Classic pong gameplay against an AI opponent
- Dynamic game area sizing
- Configurable ball and paddle speeds

### Circles
- Click circles before time runs out
- Progressive difficulty with increasing circle count
- Configurable time limits and circle amounts

### Avoider
- Mouse-controlled movement
- Dodge incoming projectiles
- Configurable spawn rates and game duration

## Features

- Borderless fullscreen window
- Transparent window support
- Smooth transitions between games
- Screen flash effects on loss
- Configurable game settings via YAML
- Mouse passthrough support

## Controls

- **Mouse**: Game controls (depending on active game)
- **F8**: Exit application

## Configuration

Game settings can be modified in `settings.yaml`:

```yaml
Settings:
  - 120.0    # Time between games (seconds)

Pong:
  - true    # Enable/disable game
  - 300.0   # Ball speed
  - 200.0   # Paddle speed
  - 200.0   # AI paddle speed

Circles:
  - true    # Enable/disable game
  - 4       # Minimum circles
  - 10      # Maximum circles
  - 5.0     # Time limit (seconds)

Avoider:
  - true    # Enable/disable game
  - 15.0    # Time limit (seconds)
  - 0.6     # Object spawn interval (seconds)
```

## Dependencies

- raylib = "5.5"
- winapi = "0.3"
- mki = "0.2.3"
- rand = "0.9.1"
- yaml-rust2 = "0.10.2"

## Building

1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Run with:
```sh
cargo run --release
```
