//! Tauri 桌面应用库

use tauri::{App, Manager};
use tracing::info;

/// 应用初始化
pub fn init_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("初始化桌面应用...");

    // 获取主窗口
    let _window = app.get_webview_window("main").unwrap();

    // TODO: 添加初始化逻辑

    Ok(())
}

/// 应用状态（可在命令中共享）
#[derive(Default)]
pub struct AppState {
    // TODO: 添加应用状态
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state() {
        let _state = AppState::default();
    }
}
