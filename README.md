# Rust Full-Stack Project

一个包含后端服务和 Tauri 桌面客户端的 Rust 项目。

## 项目结构

```
.
├── Cargo.toml              # Workspace 配置
├── README.md
├── backend/                # 后端服务
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # 后端入口
└── desktop/                # Tauri 桌面客户端
    ├── index.html          # 前端页面
    ├── src/
    │   ├── main.js         # 前端脚本
    │   └── assets/         # 静态资源
    └── src-tauri/
        ├── Cargo.toml      # Tauri Rust 配置
        ├── build.rs
        ├── tauri.conf.json # Tauri 配置
        ├── capabilities/   # Tauri 权限配置
        └── src/
            ├── main.rs     # Tauri 入口
            └── lib.rs      # Tauri 库
```

## 技术栈

- **后端**: Axum + Tokio
- **桌面端**: Tauri 2.0
- **前端**: 原生 HTML/JS (可替换为 React/Vue/Svelte)

## 运行项目

### 后端服务

```bash
cd backend
cargo run
```

后端服务将启动在 `http://127.0.0.1:3000`

### 桌面客户端

```bash
# 安装 Tauri CLI
cargo install tauri-cli

# 运行桌面应用 (开发模式)
cargo tauri dev

# 构建桌面应用
cargo tauri build
```

## 开发说明

1. **后端**: 使用 Axum Web 框架，可通过 REST API 或 WebSocket 与桌面端通信
2. **桌面端**: 使用 Tauri 2.0，前端可使用任意前端框架
3. **通信**: 桌面端可以通过 HTTP 请求与后端通信，或直接使用后端 crate 作为依赖

## 添加前端框架 (可选)

如需添加前端框架，可在 `desktop/` 目录下初始化:

```bash
cd desktop
# React + Vite
npm create vite@latest . -- --template react

# 或 Vue
npm create vite@latest . -- --template vue
```

然后更新 `tauri.conf.json` 中的 `frontendDist` 和 `devUrl` 配置。
