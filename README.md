# Windows Input Processing Research (PoC)

This repository contains a Rust-based demonstration of low-level Windows API interactions, specifically focusing on global hooks and inter-process synchronization.

## ⚠️ Disclaimer

**This project is for educational and research purposes only.** It was created to study the `SetWindowsHookExW` and `CreateMutexW` system calls. Any use of this code for malicious purposes or without consent is strictly prohibited. The author is not responsible for any misuse.

## Technical Overview

The project explores two main concepts:

1. **Process Persistence:** Using named Mutexes (`CreateMutexW`) to ensure a singleton process state and monitoring system health via a "watcher" pattern.
2. **Low-Level Input Hooks:** Implementing a `WH_KEYBOARD_LL` hook to intercept and analyze keyboard timing.

### How it works

- The `watcher` ensures the application is always running in the background.
- The `troll_script` module implements a logic that detects "fast typing" (less than 100ms between keystrokes) and simulates a "faulty hardware" behavior by swapping characters using the `SendInput` API.

## Project Structure

- `main.rs`: Handles process management, cloning, and mutex locking.
- `troll.rs`: Contains the hook logic and Win32 input manipulation.

## Requirements

- Windows OS
- Rust 1.70+
- `windows-sys` crate with `Win32_UI_Input_KeyboardAndMouse` features.

## Safety Features

- **Recursion Guard:** Uses `LLKHF_INJECTED` flag to prevent the hook from processing its own inputs.
- **Mutex Protection:** Prevents multiple instances from cluttering system memory.
