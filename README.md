# Superglide Automation Tool

A powerful desktop application for automating Superglide techniques in **Apex Legends**. This tool allows you to configure and execute precise key combinations with customizable delays and triggers.

## Overview

Superglide is an advanced movement technique in Apex Legends that requires precise timing of multiple key presses. This application streamlines the execution by automating the key sequences, making it more accessible and consistent.

## Features

### Key Configuration
- **Combo Activation Key**: Configure the main key to trigger the Superglide combination
- **Trigger Key**: Set the specific trigger key for initiating the sequence
- **Jump Key**: Full control over the jump key with adjustable hold duration
- **Crouch Key**: Customizable crouch key with configurable hold delay

### Precise Timing Control
- **Jump Hold Duration**: Set how long the jump key should be held (in milliseconds)
- **Crouch Hold Duration**: Configure the crouch key hold duration
- **Jump-to-Crouch Delay**: Fine-tune the delay between jump and crouch key presses for perfect timing

### Game Settings
- **FPS Selection**: Choose your in-game FPS setting to optimize timing synchronization
- **Activation Toggle**: Enable/disable the automation with a dedicated activation button
  - **Important**: The global key listener is only active when this button is enabled

## Getting Started

1. **Configure Your Keys**: Open the application and set up your preferred key bindings
2. **Activate the Tool**: Click the activation button to enable the global key listener
   - You must do this for the automation to start listening for your trigger key
3. **Test and Adjust**: Use [Apex Movement Superglide Trainer](https://apexmovement.tech/superglidetrainer/) to test your settings

## Tuning Your Settings

### Important Notes
- After changing any key configuration, you must **restart the global key listener** for the changes to take effect
- Click the activation button again to restart the listener

### Fine-Tuning Guide
For precise calibration of timing values:
1. Configure your keys and delays in this application
2. Visit [Apex Movement Superglide Trainer](https://apexmovement.tech/superglidetrainer/) 
3. Test your automation on the trainer website
4. Adjust settings in the application and restart the key listener
5. Repeat until you achieve consistent Superglides

## Technical Stack

- **Frontend**: SvelteKit + TypeScript + Vite
- **Desktop**: Tauri (Rust-based cross-platform app)
- **Backend**: Rust with native key hooking and automation

## Development Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

**Copyright (c) 2026 Dmitry Osin** <d@osin.pro>
