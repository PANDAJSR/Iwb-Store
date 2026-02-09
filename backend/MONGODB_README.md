# MongoDB 使用指南

## 基本查询示例

本项目提供了基本的 MongoDB 数据库查询功能。

### 安装 MongoDB

确保你的系统上安装了 MongoDB：

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install mongodb

# macOS (使用 Homebrew)
brew tap mongodb/brew
brew install mongodb-community

# Windows
# 从 https://www.mongodb.com/try/download/community 下载安装
```

### 配置环境变量

1. 复制 `.env.example` 到 `.env`：
```bash
cp .env.example .env
```

2. 修改 `.env` 文件中的 MongoDB 连接配置：
```env
# 本地 MongoDB
MONGODB_URL=mongodb://localhost:27017
MONGODB_NAME=test_db

# 或者远程 MongoDB
# MONGODB_URL=mongodb://username:password@host:port/database
```

### 运行基本查询示例

```bash
cargo run --example mongodb_basic
```

这将执行以下操作：
1. 连接到 MongoDB
2. 插入一个新用户
3. 查询所有用户
4. 根据名称查询用户
5. 更新用户信息
6. 分页查询

### 可用的查询方法

在 `src/db/mongodb.rs` 中实现了以下基本查询方法：

- `get_all_users()` - 获取所有用户
- `get_user_by_name(name)` - 根据名称查询用户
- `get_user_by_id(id)` - 根据ID查询用户
- `insert_user(user)` - 插入新用户
- `update_user(id, user)` - 更新用户信息
- `delete_user(id)` - 删除用户
- `query_users(filter)` - 根据条件查询（使用 MongoDB 查询文档）
- `get_users_paginated(page, per_page)` - 分页查询

### API 路由集成

要在你的 API 路由中使用 MongoDB，可以在 `main.rs` 中获取 `MongoDB` 实例：

```rust
// 在 web 处理函数中
async fn get_users(
    State(mongo): State<Option<db::MongoDB>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    match mongo {
        Some(mongo) => {
            let users = mongo.get_all_users().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(users))
        }
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}
```

### 数据模型

用户数据模型定义在 `src/models.rs`：

```rust
pub struct User {
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
    pub created_at: Option<DateTime<Utc>>,
}
```

### 高级查询示例

使用 `query_users` 方法可以执行复杂的 MongoDB 查询：

```rust
// 查询年龄大于 18 岁的用户
let filter = doc! { "age": { "$gt": 18 } };
let adult_users = mongo.query_users(filter).await?;

// 查询名字包含 "张" 的用户
let filter = doc! { "name": { "$regex": "张" } };
let users_with_zhang = mongo.query_users(filter).await?;
```