//! 简单的 HTTP 启动入口，提供 Swagger UI 与业务路由。
//!
//! 运行前需设置数据库连接串：
//! ```bash
//! export DATABASE_URL=postgres://user:pass@localhost:5432/dbname
//! cargo run -p biz-metadata
//! ```
use std::net::SocketAddr;

use axum::Router;
use biz_metadata::{interface::http::router::build_router, metadata_service_from_url};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url =
        std::env::var("DATABASE_URL").map_err(|_| "请设置环境变量 DATABASE_URL 以连接数据库")?;

    let service = metadata_service_from_url(&db_url).await?;
    let app: Router = build_router(service);

    let addr: SocketAddr = std::env::var("BIZ_METADATA_HTTP_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse()
        .map_err(|_| "BIZ_METADATA_HTTP_ADDR 解析失败，请使用 host:port 格式")?;

    println!("Biz Metadata HTTP listening on http://{addr}");
    println!("Swagger UI: http://{addr}/docs");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
