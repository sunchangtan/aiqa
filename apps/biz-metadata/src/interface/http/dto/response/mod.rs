pub mod biz_metadata_response;
pub mod empty_payload;
pub mod page_result_response;
pub mod result_response;

pub use biz_metadata_response::BizMetadataResponse;
pub use empty_payload::EmptyPayload;
pub use page_result_response::PageResultResponse;
pub use result_response::ResultResponse;

/// 统一响应类型别名，便于 OpenAPI 声明。
pub type BizMetadataResponseBody = ResultResponse<BizMetadataResponse>;
pub type BizMetadataPageResponseBody = ResultResponse<PageResultResponse<BizMetadataResponse>>;
pub type EmptyResponseBody = ResultResponse<()>;
