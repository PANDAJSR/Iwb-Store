use anyhow::Result;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Database, Collection,
};
use tracing::info;
use crate::models::{User, AppMetadata};
use futures_util::stream::TryStreamExt;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MongoDB {
    client: Client,
    database: Database,
}

impl MongoDB {
    /// 创建 MongoDB 连接
    pub async fn new(database_url: &str, database_name: &str) -> Result<Self> {
        // 配置客户端选项
        let mut client_options = ClientOptions::parse(database_url).await?;

        // 设置连接池大小
        client_options.max_pool_size = Some(10);

        // 创建客户端
        let client = Client::with_options(client_options)?;

        // 获取数据库
        let database = client.database(database_name);

        // 测试连接
        client
            .database("admin")
            .run_command(doc! { "ping": 1 })
            .await?;

        info!("成功连接到 MongoDB 数据库: {}", database_name);

        Ok(Self { client, database })
    }

    /// 获取用户集合
    pub fn users_collection(&self) -> Collection<User> {
        self.database.collection::<User>("users")
    }

    /// 基本查询 - 获取所有用户
    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let collection = self.users_collection();
        let users = collection.find(doc! {}).await?.try_collect().await?;
        Ok(users)
    }

    /// 根据名字查询用户
    pub async fn get_user_by_name(&self, name: &str) -> Result<Option<User>> {
        let collection = self.users_collection();
        let user = collection.find_one(doc! { "name": name }).await?;
        Ok(user)
    }

    /// 插入新用户
    pub async fn insert_user(&self, user: User) -> Result<User> {
        let collection = self.users_collection();
        let mut new_user = user;

        // 设置创建时间
        new_user.created_at = Some(chrono::Utc::now());

        let result = collection.insert_one(&new_user).await?;

        // 获取插入后的用户（包含生成的ID）
        if let Some(id) = result.inserted_id.as_object_id() {
            new_user.id = Some(id);
        }

        Ok(new_user)
    }

    /// 更新用户
    pub async fn update_user(&self, id: &str, user: User) -> Result<Option<User>> {
        let collection = self.users_collection();
        let object_id = mongodb::bson::oid::ObjectId::parse_str(id)?;

        let result = collection
            .replace_one(
                doc! { "_id": object_id },
                user
            )
            .await?;

        if result.matched_count > 0 {
            self.get_user_by_id(&object_id.to_string()).await
        } else {
            Ok(None)
        }
    }

    /// 根据ID查询用户
    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let collection = self.users_collection();
        let object_id = mongodb::bson::oid::ObjectId::parse_str(id)?;
        let user = collection.find_one(doc! { "_id": object_id }).await?;
        Ok(user)
    }

    /// 删除用户
    pub async fn delete_user(&self, id: &str) -> Result<bool> {
        let collection = self.users_collection();
        let object_id = mongodb::bson::oid::ObjectId::parse_str(id)?;

        let result = collection
            .delete_one(doc! { "_id": object_id })
            .await?;

        Ok(result.deleted_count > 0)
    }

    /// 高级查询 - 根据条件查询用户
    pub async fn query_users(&self, filter: Document) -> Result<Vec<User>> {
        let collection = self.users_collection();
        let users = collection.find(filter).await?.try_collect().await?;
        Ok(users)
    }

    /// 分页查询
    pub async fn get_users_paginated(
        &self,
        page: u64,
        per_page: u64,
    ) -> Result<Vec<User>> {
        let collection = self.users_collection();

        let options = mongodb::options::FindOptions::builder()
            .skip((page - 1) * per_page)
            .limit(per_page as i64)
            .build();

        let users = collection.find(doc! {}).with_options(options).await?.try_collect().await?;
        Ok(users)
    }

    /// 获取 apps-metadata 集合
    pub fn apps_metadata_collection(&self) -> Collection<AppMetadata> {
        self.database.collection::<AppMetadata>("apps-metadata")
    }

    /// 获取所有 apps metadata（使用灵活的Map结构避免字段缺失错误）
    pub async fn get_all_apps_metadata_flexible(&self) -> Result<Vec<HashMap<String, mongodb::bson::Bson>>> {
        let collection = self.database.collection::<mongodb::bson::Document>("apps-metadata");
        let mut cursor = collection.find(doc! {}).await?;
        let mut results = Vec::new();

        while let Some(document) = cursor.try_next().await? {
            let mut map = HashMap::new();
            for (key, value) in document {
                map.insert(key, value);
            }
            results.push(map);
        }

        Ok(results)
    }
}

/// 创建 MongoDB 客户端的辅助函数
pub async fn create_mongo_client(database_url: &str, database_name: &str) -> Result<MongoDB> {
    MongoDB::new(database_url, database_name).await
}