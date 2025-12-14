use serde::Deserialize;
use utoipa::IntoParams;

/// 删除 BizMetadata 时需要的参数（乐观锁）。
#[derive(Debug, Deserialize, IntoParams, utoipa::ToSchema)]
pub struct DeleteBizMetadataParams {
    /// 版本号，必须与服务端当前版本一致。
    pub version: i32,
}
