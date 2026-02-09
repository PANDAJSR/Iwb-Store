//! Tauri 桌面应用入口

// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
    tracing_subscriber::fmt::init();

    // 创建上下文配置
    let context = tauri::generate_context!(
        "../src-tauri/tauri.conf.json"
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .run(context)
        .expect("运行 Tauri 应用时出错");
}
