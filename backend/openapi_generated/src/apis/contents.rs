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
pub enum GetContentResponse {
    /// コンテンツの正常な取得
    Status200
    (models::Content)
    ,
    /// リクエストパラメーターが不正である
    Status400
    (models::BadRequestProblem)
    ,
    /// コンテンツが存在しないもしくはメタデータを取得できない
    Status404
    (models::ContentNotFoundProblem)
    ,
    /// サーバー内部で予期しないエラーが発生した
    Status500
    (models::InternalServerErrorProblem)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListContentsResponse {
    /// コンテンツ一覧の正常な取得
    Status200
    (models::ContentPage)
    ,
    /// リクエストパラメーターが不正である
    Status400
    (models::BadRequestProblem)
    ,
    /// サーバー内部で予期しないエラーが発生した
    Status500
    (models::InternalServerErrorProblem)
}




/// Contents
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Contents<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// コンテンツの取得.
    ///
    /// GetContent - GET /api/v0/contents/{contentId}
    async fn get_content(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      path_params: &models::GetContentPathParams,
    ) -> Result<GetContentResponse, E>;

    /// コンテンツ一覧の取得.
    ///
    /// ListContents - GET /api/v0/contents
    async fn list_contents(
    &self,
    
    method: &Method,
    host: &Host,
    cookies: &CookieJar,
      query_params: &models::ListContentsQueryParams,
    ) -> Result<ListContentsResponse, E>;
}
