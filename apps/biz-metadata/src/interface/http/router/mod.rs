#[path = "router.rs"]
pub mod routes;

// 暴露默认路由构建入口，便于上层使用。
pub use routes::build_router;
