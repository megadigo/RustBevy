# Bevy Platformer Game

A simple 2D platformer game built with Bevy Engine in Rust.

## Prerequisites

You need to have Rust installed on your system. If you don't have Rust installed:

1. **Install Rust**: Go to [https://rustup.rs/](https://rustup.rs/) and follow the installation instructions for Windows
   - Download and run `rustup-init.exe`
   - Follow the on-screen instructions
   - Restart your terminal/PowerShell after installation

2. **Verify Installation**: After installation, verify Rust is working:
   ```powershell
   rustc --version
   cargo --version
   ```

## How to Run

1. **Clone or navigate to the project directory**:
   ```powershell
   cd c:\Git\RustBevy
   ```

2. **Build and run the game**:
   ```powershell
   cargo run
   ```

   The first build may take several minutes as it downloads and compiles dependencies.

## Game Controls

- **Move Left**: ← Arrow Key or A
- **Move Right**: → Arrow Key or D  
- **Jump**: Space, ↑ Arrow Key, or W

## Game Features

- **Player Character**: Blue square that can move and jump
- **Platforms**: Gray platforms at different heights
- **Physics**: Gravity and collision detection
- **Respawn**: Player respawns at the top if they fall off the screen

## Game Mechanics

- The player is affected by gravity and will fall if not on a platform
- Player can only jump when grounded (touching a platform)
- Collision detection prevents the player from passing through platforms
- Player movement is smooth and responsive

## Technical Details

- **Engine**: Bevy 0.14
- **Language**: Rust
- **Platform**: Cross-platform (Windows, macOS, Linux)
- **Graphics**: 2D sprites with basic shapes

## Customization

You can easily modify the game by changing constants in `src/main.rs`:

- `PLAYER_SPEED`: How fast the player moves horizontally
- `JUMP_SPEED`: How high the player jumps
- `GRAVITY`: How strong the gravity effect is
- `WINDOW_WIDTH` / `WINDOW_HEIGHT`: Game window dimensions

## Expanding the Game

This is a basic foundation that can be extended with:

- Sprites and animations instead of colored rectangles
- Sound effects and music
- Enemies and hazards
- Collectible items
- Multiple levels
- Better graphics and particle effects
- Game states (menu, pause, game over)