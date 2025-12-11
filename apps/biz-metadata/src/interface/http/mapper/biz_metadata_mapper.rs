use crate::application::service::biz_metadata::{
    BizMetadataQueryRequest,
    command::{CreateBizMetadataCommand, FieldUpdate, UpdateBizMetadataCommand},
};
use crate::domain::biz_metadata::BizMetadata;
use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, BizMetadataType, DataClass,
};
use crate::interface::http::dto::request::{
    BizMetadataListParams, CreateBizMetadataRequest, UpdateBizMetadataRequest,
};
use crate::interface::http::dto::response::{BizMetadataResponse, PageResultResponse};
use crate::interface::http::mapper::error_mapper::HttpError;
use domain_core::expression::{Expression, QueryOptions};
use domain_core::pagination::{Page, PageResult};

/// BizMetadata 相关 DTO 与领域模型的转换器。
pub struct BizMetadataDtoMapper;

impl BizMetadataDtoMapper {
    /// 请求载荷转换为创建命令。
    pub fn map_to_create_command(
        payload: CreateBizMetadataRequest,
    ) -> Result<CreateBizMetadataCommand, HttpError> {
        let meta_type = Self::map_meta_type(&payload.meta_type)?;
        let data_class = Self::map_data_class(&payload.data_class)?;
        let status = payload
            .status
            .as_deref()
            .map(Self::map_status)
            .transpose()?;

        Ok(CreateBizMetadataCommand {
            code: payload.code,
            name: payload.name,
            description: payload.description,
            metadata_type: meta_type,
            data_class,
            value_type: payload.value_type,
            owner_id: payload.owner_id.map(BizMetadataId::new),
            unit: payload.unit,
            is_identifier: payload.is_identifier,
            status,
        })
    }

    /// 请求载荷转换为更新命令。
    pub fn map_to_update_command(
        id: i64,
        payload: UpdateBizMetadataRequest,
    ) -> Result<UpdateBizMetadataCommand, HttpError> {
        let meta_type = payload
            .meta_type
            .as_deref()
            .map(Self::map_meta_type)
            .transpose()?;
        let data_class = payload
            .data_class
            .as_deref()
            .map(Self::map_data_class)
            .transpose()?;
        let status = payload
            .status
            .as_deref()
            .map(Self::map_status)
            .transpose()?;

        Ok(UpdateBizMetadataCommand {
            id: BizMetadataId::new(id),
            name: payload.name,
            metadata_type: meta_type,
            description: match payload.description {
                Some(Some(desc)) => FieldUpdate::Set(desc),
                Some(None) => FieldUpdate::Clear,
                None => FieldUpdate::Keep,
            },
            data_class,
            value_type: payload.value_type,
            unit: match payload.unit {
                Some(Some(val)) => FieldUpdate::Set(val),
                Some(None) => FieldUpdate::Clear,
                None => FieldUpdate::Keep,
            },
            owner_id: match payload.owner_id {
                Some(Some(val)) => FieldUpdate::Set(BizMetadataId::new(val)),
                Some(None) => FieldUpdate::Clear,
                None => FieldUpdate::Keep,
            },
            is_identifier: payload.is_identifier,
            status,
        })
    }

    /// 列表查询参数转查询请求。
    pub fn map_to_query_request(params: BizMetadataListParams) -> BizMetadataQueryRequest {
        BizMetadataQueryRequest {
            expression: Expression::True,
            options: QueryOptions {
                limit: params.limit,
                offset: params.offset,
                order_bys: vec![],
            },
        }
    }

    /// 领域对象转响应 DTO。
    pub fn map_to_response(entity: BizMetadata) -> BizMetadataResponse {
        BizMetadataResponse::from(entity)
    }

    /// 分页结果转响应 DTO。
    pub fn map_to_page_response(
        page: PageResult<BizMetadata>,
    ) -> PageResultResponse<BizMetadataResponse> {
        let total_count = page.total_count();
        let page_index = page.page_index();
        let page_size = page.page_size();
        let index_from = page.index_from();
        let items = page
            .into_items()
            .into_iter()
            .map(BizMetadataResponse::from)
            .collect::<Vec<_>>();
        let mapped_page = PageResult::new(
            items,
            total_count,
            page_index,
            Some(page_size),
            Some(index_from),
        );
        PageResultResponse::from_page(mapped_page)
    }

    fn map_meta_type(raw: &str) -> Result<BizMetadataType, HttpError> {
        match raw {
            "attribute" | "field" => Ok(BizMetadataType::Attribute),
            "entity" => Ok(BizMetadataType::Entity),
            "event" => Ok(BizMetadataType::Event),
            other => Err(HttpError::bad_request(format!(
                "invalid meta_type: {other}"
            ))),
        }
    }

    fn map_data_class(raw: &str) -> Result<DataClass, HttpError> {
        DataClass::try_from(raw).map_err(|err| HttpError::bad_request(err.to_string()))
    }

    fn map_status(raw: &str) -> Result<BizMetadataStatus, HttpError> {
        match raw {
            "active" => Ok(BizMetadataStatus::Active),
            "deprecated" => Ok(BizMetadataStatus::Deprecated),
            "draft" => Ok(BizMetadataStatus::Draft),
            other => Err(HttpError::bad_request(format!("invalid status: {other}"))),
        }
    }
}
