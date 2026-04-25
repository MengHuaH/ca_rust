# CA_Rust

一个基于Rust和Axum框架构建的现代化Web API服务，采用领域驱动设计(DDD)架构。

## 🚀 特性

- **现代化架构**: 采用分层架构(DDD)设计，清晰的职责分离
- **高性能**: 基于Tokio异步运行时和Axum Web框架
- **数据库支持**: 使用SeaORM支持多种数据库(PostgreSQL/MySQL/SQLite)
- **API文档**: 自动生成Swagger/OpenAPI文档
- **完善的日志**: 结构化日志记录，支持文件和控制台输出
- **用户管理**: 完整的用户CRUD操作，支持软删除
- **输入验证**: 完善的DTO验证和错误处理机制

## 🏗️ 架构设计

```
src/
├── domain/          # 领域层 - 业务逻辑和实体
├── application/     # 应用层 - 用例编排和服务
├── api/            # 表现层 - Web API和路由
└── infrastructure/ # 基础设施层 - 数据库、配置等
```

## 📦 技术栈

- **Web框架**: Axum 0.7
- **异步运行时**: Tokio 1.0
- **ORM**: SeaORM 0.12
- **数据库**: PostgreSQL (支持MySQL/SQLite)
- **API文档**: Utoipa + Swagger UI
- **日志**: tracing + tracing-subscriber
- **验证**: validator crate
- **配置**: 环境变量 + dotenvy

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 12+

### 安装运行

1. **克隆项目**

```bash
git clone <repository-url>
cd ca_rust
```

2. **配置环境变量**

```bash
cp .env.example .env
# 编辑 .env 文件配置数据库连接
```

3. **运行数据库迁移**

数据库迁移工具支持以下操作：

**执行迁移（默认）**

```bash
cargo run --bin migrate
# 或
cargo run --bin migrate -- --action migrate
```

**回滚迁移**

```bash
# 回滚到指定版本
cargo run --bin migrate -- --action rollback --target 001

# 回滚最后一个迁移
cargo run --bin migrate -- --action rollback
```

**创建迁移模板**

```bash
cargo run --bin migrate -- --action create --name your_table_name
```

**查看帮助信息**

```bash
cargo run --bin migrate -- --help
```

**调试模式**

```bash
$env:RUST_LOG="debug"; cargo run --bin migrate
```

4. **启动服务**

```bash
cargo run
```

5. **访问API文档**

- Swagger UI: http://localhost:3000/swagger-ui
- OpenAPI JSON: http://localhost:3000/api-docs/openapi.json

## 📚 API接口

### 用户管理

| 方法   | 路径                          | 描述             |
| ------ | ----------------------------- | ---------------- |
| POST   | `/api/users/create`           | 创建用户         |
| PUT    | `/api/users/update/{user_id}` | 更新用户         |
| DELETE | `/api/users/delete/{user_id}` | 删除用户(软删除) |

### 请求示例

**创建用户**

```bash
curl -X POST "http://localhost:3000/api/users/create" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "张三",
    "phone": "13800138000",
    "email": "zhangsan@example.com",
    "password": "password123"
  }'
```

## ⚙️ 配置说明

### 日志配置

- **控制台输出**: 美化格式，便于开发调试
- **文件输出**: JSON格式，每天轮转，保留7天
- **日志路径**: `./logs/CA.log`

## 🛠️ 开发指南

### 项目结构

```
src/
├── domain/entities/         # 领域实体(充血模型)
├── application/users/       # 用户应用服务
│   ├── command/            # 命令(CQRS模式)
│   └── queries/            # 查询
├── api/users/              # 用户API接口
│   ├── handlers/           # 请求处理器
│   └── routes.rs           # 路由配置
└── infrastructure/         # 基础设施
    ├── database/           # 数据访问层
    └── common/             # 通用工具
```

### 添加新功能

1. **定义领域实体** (`domain/entities/`)
2. **创建应用服务** (`application/`)
3. **实现API接口** (`api/`)
4. **添加数据访问** (`infrastructure/database/`)

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 高性能Web框架
- [SeaORM](https://www.sea-ql.org/SeaORM/) - 异步ORM框架
- [Tokio](https://tokio.rs/) - 异步运行时

---

**开发中** - 更多功能正在开发中！
