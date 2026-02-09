//! 后端服务入口

mod db;
mod models;

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

    // 初始化 MongoDB 连接（可选）
    let _mongo_client = init_mongodb().await;

    // 构建路由
    let app = create_router();

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("服务监听于: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 初始化 MongoDB 连接
async fn init_mongodb() -> Option<db::MongoDB> {
    match dotenvy::var("MONGODB_URL") {
        Ok(database_url) => {
            let database_name = dotenvy::var("MONGODB_NAME").unwrap_or_else(|_| "test_db".to_string());

            match db::create_mongo_client(&database_url, &database_name).await {
                Ok(client) => {
                    info!("MongoDB 连接成功");
                    Some(client)
                }
                Err(e) => {
                    tracing::warn!("MongoDB 连接失败: {}，使用内存模式运行", e);
                    None
                }
            }
        }
        Err(_) => {
            tracing::info!("未检测到 MongoDB 配置，使用内存模式运行");
            None
        }
    }
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
