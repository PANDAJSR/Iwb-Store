//! 后端服务入口

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("启动后端服务...");

    // 构建路由
    let app = create_router();

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("服务监听于: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 创建应用路由
fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
}

/// 根路由处理
async fn root() -> &'static str {
    "Hello, Backend!"
}

/// 健康检查
async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "OK");
    }
}
