# Keyboard Light

A lightweight Windows desktop utility that displays a subtle white gradient overlay at the bottom of the screen,
providing illumination for keyboards without backlight functionality. Built with Rust and Tauri for minimal resource usage.
Just a simple application for personal use.

## Features

- Transparent Overlay: Displays a semi-transparent white gradient at the bottom of your screen
- Click-Through: The overlay doesn't interfere with your work - clicks pass through to applications underneath
- System Tray Control:
  - Left-click the tray icon to toggle the overlay visibility
  - Middle-click the tray icon to exit the application
- Global Shortcut: Press `Ctrl+Alt+Space` to toggle the overlay from anywhere
- Minimal Footprint: Runs silently in the background with no taskbar presence

## Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)

## Build Steps

```bash
# Install dependencies
npm install

# Build the application
npm run tauri build
```

The compiled executable will be in  `src-tauri/target/release/keyboard-light.exe`

## License

This project is licensed under the MIT License.