use crate::application::service::biz_metadata_alias::{
    AliasFieldUpdate, BizMetadataAliasQueryRequest, CreateBizMetadataAliasCommand,
    UpdateBizMetadataAliasCommand,
};
use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::domain::biz_metadata_alias::BizMetadataAlias;
use crate::domain::biz_metadata_alias::value_object::{
    AliasSource, AliasWeight, BizMetadataAliasId, LanguageCode,
};
use crate::interface::http::dto::request::{
    BizMetadataAliasListParams, CreateBizMetadataAliasRequest, UpdateBizMetadataAliasRequest,
};
use crate::interface::http::dto::response::{BizMetadataAliasResponse, PageResultResponse};
use crate::interface::http::mapper::error_mapper::HttpError;
use domain_core::expression::{Expression, QueryOptions};
use domain_core::pagination::{Page, PageResult};

/// BizMetadataAlias 相关 DTO 与领域模型的转换器。
pub struct BizMetadataAliasDtoMapper;

impl BizMetadataAliasDtoMapper {
    pub fn map_to_create_command(
        payload: CreateBizMetadataAliasRequest,
    ) -> Result<CreateBizMetadataAliasCommand, HttpError> {
        let source = payload
            .source
            .as_deref()
            .map(AliasSource::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;
        let weight = payload
            .weight
            .map(AliasWeight::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;
        let language = payload
            .language
            .map(LanguageCode::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        Ok(CreateBizMetadataAliasCommand {
            metadata_id: BizMetadataId::new(payload.metadata_id),
            alias: payload.alias,
            source,
            weight,
            is_primary: payload.is_primary,
            language,
        })
    }

    pub fn map_to_update_command(
        id: i64,
        payload: UpdateBizMetadataAliasRequest,
    ) -> Result<UpdateBizMetadataAliasCommand, HttpError> {
        let source = payload
            .source
            .as_deref()
            .map(AliasSource::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;
        let weight = payload
            .weight
            .map(AliasWeight::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;
        let language = payload
            .language
            .map(LanguageCode::new)
            .transpose()
            .map_err(|e| HttpError::bad_request(e.to_string()))?;

        Ok(UpdateBizMetadataAliasCommand {
            id: BizMetadataAliasId::new(id),
            metadata_id: payload.metadata_id.map(BizMetadataId::new),
            alias: payload.alias.map(AliasFieldUpdate::Set).unwrap_or_default(),
            source,
            weight,
            is_primary: payload.is_primary,
            language,
        })
    }

    pub fn map_to_query_request(
        params: BizMetadataAliasListParams,
    ) -> BizMetadataAliasQueryRequest {
        // TODO: expression building when filters used; keep Expression::True for now.
        BizMetadataAliasQueryRequest {
            expression: Expression::True,
            options: QueryOptions {
                limit: params.limit,
                offset: params.offset,
                order_bys: vec![],
            },
        }
    }

    pub fn map_to_response(domain: BizMetadataAlias) -> BizMetadataAliasResponse {
        BizMetadataAliasResponse::from(domain)
    }

    pub fn map_to_page_response(
        page: PageResult<BizMetadataAlias>,
    ) -> PageResultResponse<BizMetadataAliasResponse> {
        let total_count = page.total_count();
        let page_index = page.page_index();
        let page_size = page.page_size();
        let index_from = page.index_from();
        let items = page
            .into_items()
            .into_iter()
            .map(BizMetadataAliasResponse::from)
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
}
