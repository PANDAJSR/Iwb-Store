use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // 创建 .env 文件（如果不存在）
    dotenvy::from_filename(".env").ok();

    // 数据库连接字符串
    let database_url = std::env::var("MONGODB_URL")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let database_name = std::env::var("MONGODB_NAME")
        .unwrap_or_else(|_| "test_db".to_string());

    info!("连接到 MongoDB: {}", database_url);

    // 示例：插入用户
    let new_user = User {
        id: None,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: Some(25),
        created_at: None,
    };

    let inserted_user = mongo.insert_user(new_user).await?;
    info!("插入用户成功: {:?}", inserted_user);

    // 示例：查询所有用户
    let users = mongo.get_all_users().await?;
    info!("找到 {} 个用户", users.len());
    for user in &users {
        info!("用户: {} ({})", user.name, user.email);
    }

    // 示例：根据名字查询用户
    if let Some(found_user) = mongo.get_user_by_name("张三").await? {
        info!("找到用户 '张三': {:?}", found_user);

        // 示例：更新用户
        let mut updated_user = found_user.clone();
        updated_user.age = Some(26);

        if let Some(id) = &found_user.id {
            mongo.update_user(&id.to_string(), updated_user).await?;
            info!("用户更新成功");
        }
    }

    // 示例：分页查询
    info!("\n分页查询结果:");
    let page1_users = mongo.get_users_paginated(1, 2).await?;
    for (i, user) in page1_users.iter().enumerate() {
        info!("第 1 页，第 {} 个用户: {}", i + 1, user.name);
    }

    info!("MongoDB 基本操作示例完成！");
    Ok(())
}