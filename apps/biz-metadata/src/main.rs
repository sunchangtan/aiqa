//! 简单的 HTTP 启动入口，提供 Swagger UI 与业务路由。
//!
//! 运行前需设置数据库连接串：
//! ```bash
//! export DATABASE_URL=postgres://user:pass@localhost:5432/dbname
//! cargo run -p biz-metadata
//! ```
use std::net::SocketAddr;

use biz_metadata::{build_alias_service, build_service, interface::http::router::build_router};
use sea_orm::Database;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 尝试从项目根目录的 .env 加载环境变量（如果没有 .env 会忽略错误）
    let _ = dotenvy::dotenv();

    let db_url =
        std::env::var("DATABASE_URL").map_err(|_| "请设置环境变量 DATABASE_URL 以连接数据库")?;

    let db = Database::connect(&db_url).await?;
    let biz_metadata_service = build_service(db.clone());
    let biz_metadata_alias_service = build_alias_service(db);
    let app_layer = build_router(biz_metadata_service, biz_metadata_alias_service);

    let addr: SocketAddr = std::env::var("BIZ_METADATA_HTTP_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .map_err(|_| "BIZ_METADATA_HTTP_ADDR 解析失败，请使用 host:port 格式")?;

    println!("Biz Metadata HTTP listening on http://{addr}");
    println!("Swagger UI: http://{addr}/docs");

    let listener = TcpListener::bind(addr).await?;
    let make_service = app_layer.into_make_service();
    axum::serve(listener, make_service).await?;

    Ok(())
}
