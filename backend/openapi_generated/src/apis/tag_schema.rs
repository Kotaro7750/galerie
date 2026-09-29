use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use headers::Host;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTagSchemaResponse {
    /// 現在有効なタグスキーマ
    Status200
    (models::TagSchema)
    ,
    /// サーバー内部で予期しないエラーが発生した
    Status500
    (models::InternalServerErrorProblem)
}




/// TagSchema
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait TagSchema<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// 有効なタグスキーマの取得.
    ///
    /// GetTagSchema - GET /api/v0/tag-schema
    async fn get_tag_schema(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
    ) -> Result<GetTagSchemaResponse, E>;
}
