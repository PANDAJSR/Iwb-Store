use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use futures_util::stream::{StreamExt, TryStreamExt};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
    age: u32,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 连接 MongoDB
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // 选择数据库和集合
    let db = client.database("test_db");
    let users: Collection<User> = db.collection("users");

    println!("连接 MongoDB 成功!");

    // 插入示例用户
    let user_data = User {
        id: None,
        name: "测试用户".to_string(),
        email: "test@example.com".to_string(),
        age: 25,
        created_at: Utc::now(),
    };

    let insert_result = users.insert_one(user_data).await?;
    println!("插入用户成功，ID: {:?}", insert_result.inserted_id);

    // 查询所有用户
    let cursor = users.find(doc! {}).await?;
    let users_list: Vec<User> = cursor.try_collect().await?;

    println!("\n找到 {} 个用户:", users_list.len());
    for (i, user) in users_list.iter().enumerate() {
        println!("  {}. 姓名: {}，邮箱: {}，年龄: {}",
            i + 1,
            user.name,
            user.email,
            user.age
        );
    }

    // 条件查询 - 年龄大于 20
    let mut cursor = users.find(doc! { "age": { "$gt": 20 } }).await?;
    println!("\n年龄大于 20 岁的用户:");
    while let Some(user) = cursor.try_next().await? {
        println!("  - {} ({})", user.name, user.age);
    }

    // 更新用户 - 第一个用户年龄加10岁
    if let Some(first_user) = users_list.first() {
        if let Some(id) = first_user.id {
            let filter = doc! { "_id": id };
            let update = doc! { "$inc": { "age": 10 } };
            let update_result = users.update_one(filter, update).await?;
            println!("\n更新用户成功，修改记录数: {}", update_result.modified_count);
        }
    }

    // 删除用户
    let delete_result = users.delete_one(doc! { "email": "test@example.com" }).await?;
    println!("删除用户 {} 个", delete_result.deleted_count);

    println!("\nMongoDB 基本查询示例完成!");
    Ok(())
}