use crate::application::service::biz_metadata::{
    BizMetadataQueryRequest,
    command::{CreateBizMetadataCommand, FieldUpdate, UpdateBizMetadataCommand},
};
use crate::domain::biz_metadata::BizMetadata;
use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, DataClass, ObjectType, Source, Version,
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
        let object_type = ObjectType::new(&payload.object_type)
            .map_err(|e| HttpError::bad_request(e.to_string()))?;
        if object_type != ObjectType::Feature
            && (payload.data_class.is_some()
                || payload.value_type.is_some()
                || payload.unit.is_some())
        {
            return Err(HttpError::bad_request(
                "non-feature object_type must not provide data_class/value_type/unit",
            ));
        }
        if object_type == ObjectType::Feature
            && (payload.data_class.is_none() || payload.value_type.is_none())
        {
            return Err(HttpError::bad_request(
                "object_type=feature requires data_class and value_type",
            ));
        }

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
        let source = payload
            .source
            .as_deref()
            .map(Self::map_source)
            .transpose()?;

        Ok(CreateBizMetadataCommand {
            code: payload.code,
            name: payload.name,
            description: payload.description,
            object_type,
            parent_id: payload.parent_id.map(BizMetadataId::new),
            data_class,
            value_type: payload.value_type,
            unit: payload.unit,
            status,
            source,
        })
    }

    /// 请求载荷转换为更新命令。
    pub fn map_to_update_command(
        id: i64,
        payload: UpdateBizMetadataRequest,
    ) -> Result<UpdateBizMetadataCommand, HttpError> {
        let version =
            Version::new(payload.version).map_err(|e| HttpError::bad_request(e.to_string()))?;
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
        let source = payload
            .source
            .as_deref()
            .map(Self::map_source)
            .transpose()?;

        Ok(UpdateBizMetadataCommand {
            id: BizMetadataId::new(id),
            version,
            name: payload.name,
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
            parent_id: match payload.parent_id {
                Some(Some(val)) => FieldUpdate::Set(BizMetadataId::new(val)),
                Some(None) => FieldUpdate::Clear,
                None => FieldUpdate::Keep,
            },
            status,
            source,
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

    fn map_data_class(raw: &str) -> Result<DataClass, HttpError> {
        DataClass::try_from(raw).map_err(|err| HttpError::bad_request(err.to_string()))
    }

    fn map_status(raw: &str) -> Result<BizMetadataStatus, HttpError> {
        BizMetadataStatus::try_from(raw).map_err(|err| HttpError::bad_request(err.to_string()))
    }

    fn map_source(raw: &str) -> Result<Source, HttpError> {
        Source::new(raw).map_err(|err| HttpError::bad_request(err.to_string()))
    }
}
