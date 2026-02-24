# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri 2 desktop application for refrigerator appearance inspection (冰箱外观检测系统). It combines a Vue 3 + TypeScript frontend with a Rust backend.

**Tech Stack:**
- Frontend: Vue 3 + TypeScript + Vite
- Backend: Rust + Tauri 2
- UI Framework: Element Plus (Chinese locale)
- CSS Framework: UnoCSS (utility-first atomic CSS)
- State Management: Pinia
- Routing: Vue Router

## Development Commands

### Frontend Development
```bash
npm run dev          # Start Vite dev server (port 1420)
npm run build        # Type-check and build frontend
npm run preview      # Preview production build
```

### Tauri Development
```bash
npm run tauri dev    # Run Tauri app in development mode
npm run tauri build  # Build production app bundle
```

The dev server runs on `http://localhost:1420` with HMR on port 1421.

### Rust Backend
```bash
cd src-tauri
cargo build          # Build Rust backend
cargo check          # Check Rust code without building
cargo clippy         # Run Rust linter
```

## Architecture

### Frontend Structure

The frontend follows a **module-based architecture** where each feature is organized as a self-contained module:

```
src/
├── modules/
│   ├── detection/       # Real-time appearance detection (实时外观检测)
│   ├── detecConfig/     # Detection configuration (检测配置)
│   ├── statistics/      # Data statistics (数据统计)
│   ├── production/      # Product information (产品信息)
│   ├── settings/        # System settings (系统设置)
│   │   ├── components/  # Settings sub-components
│   │   ├── models/      # TypeScript interfaces/types
│   │   └── store/       # Pinia stores for settings
│   └── layout/          # Layout components
├── router/              # Vue Router configuration
└── main.ts              # App entry point
```

**Module Organization Pattern:**
- Each module should contain its own components, models (TypeScript interfaces), and stores (Pinia)
- Place shared types in `models/` directory within the module
- Place state management in `store/` directory within the module
- Break down complex features into sub-components in `components/` directory

### Backend Structure

The Rust backend is organized into feature modules:

```
src-tauri/src/
├── lib.rs              # Main entry point, plugin initialization
├── main.rs             # Binary entry point
├── logging/            # Logging system module
│   ├── mod.rs          # Module exports
│   ├── config.rs       # LogConfig struct and persistence
│   ├── logs.rs         # Tauri plugin initialization with fern
│   └── commands.rs     # Tauri commands for log management
├── menu.rs             # Application menu (currently commented out)
└── tray.rs             # System tray (currently commented out)
```

**Logging System:**
The application uses a custom logging system built with `fern` and `tauri-plugin-log`:
- Configuration is persisted to disk at `{config_dir}/easydesktopapp/log_config.json`
- Supports configurable log levels, file rotation, max file size, and stdout output
- Exposes three Tauri commands: `get_log_config`, `update_log_config`, `reset_log_config`
- Log files are stored in the platform-specific app log directory

### Frontend-Backend Communication

Communication happens via **Tauri commands** (IPC):
- Frontend calls Rust functions using `@tauri-apps/api`
- Commands are registered in `lib.rs` using `tauri::generate_handler![]`
- All commands are async and return `Result<T, String>`

Example command registration in `src-tauri/src/lib.rs:36-40`:
```rust
.invoke_handler(tauri::generate_handler![
    crate::logging::commands::get_log_config,
    crate::logging::commands::update_log_config,
    crate::logging::commands::reset_log_config
])
```

## CSS and Styling Rules

**CRITICAL: Use UnoCSS exclusively for styling**
- Use UnoCSS atomic utility classes directly in templates
- Avoid traditional CSS class names and separate stylesheets
- Follow utility-first principles (e.g., `class="flex items-center gap-4 p-4"`)
- UnoCSS is configured via `@unocss/vite` plugin in `vite.config.ts`

## Configuration Files

### Tauri Configuration (`src-tauri/tauri.conf.json`)
- Product name: "easydesktopapp"
- Identifier: "com.robin.easydesktopapp"
- Window title: "冰箱外观检测系统"
- Default window size: 1600x900
- Custom window decorations disabled (`decorations: false`)
- Dev server: `http://localhost:1420`
- Build output: `../dist`

### Build Configuration
- **Minification disabled** in production (`vite.config.ts:12`) for easier debugging
- **Source maps enabled** for production builds
- Vite ignores `src-tauri` directory to prevent watch conflicts

## Adding New Features

### Adding a New Tauri Command

1. Create the command function in appropriate module (e.g., `src-tauri/src/module/commands.rs`)
2. Add `#[command]` attribute and make it async
3. Return `Result<T, String>` for error handling
4. Register in `lib.rs` invoke_handler
5. Export from module's `mod.rs`

### Adding a New Frontend Module

1. Create directory under `src/modules/`
2. Create main component (e.g., `ModuleName.vue`)
3. Add route in `src/router/index.ts` with Chinese `meta.title`
4. Create `models/` subdirectory for TypeScript interfaces
5. Create `store/` subdirectory for Pinia stores if needed
6. Create `components/` subdirectory for sub-components

### Adding a New Rust Module

1. Create module directory under `src-tauri/src/`
2. Create `mod.rs` to export public items
3. Add `mod module_name;` in `lib.rs`
4. If exposing commands, create `commands.rs` and register handlers

## Important Notes

- The application uses **Chinese language** for UI text and comments
- Element Plus is configured with Chinese locale (`zhCn`)
- Window has custom decorations disabled - implement custom title bar if needed
- DevTools are enabled in development (`tauri.conf.json:23`)
- Log configuration persists across app restarts via JSON file in system config directory
