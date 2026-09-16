use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use galerie_api::models::InternalServerErrorProblem;
use http::StatusCode;
use http::header::{CONTENT_TYPE, SET_COOKIE};

use crate::domain::content_access::ContentAccessCookie;
use crate::usecase::{ClearContentAccessUseCase, ConfigureContentAccessUseCase};

#[derive(Clone)]
pub(crate) struct ContentAccessController {
    configure_content_access: ConfigureContentAccessUseCase,
    clear_content_access: ClearContentAccessUseCase,
}

impl ContentAccessController {
    pub(crate) fn new(
        configure_content_access: ConfigureContentAccessUseCase,
        clear_content_access: ClearContentAccessUseCase,
    ) -> Self {
        Self {
            configure_content_access,
            clear_content_access,
        }
    }

    pub(crate) fn router(self) -> axum::Router {
        axum::Router::new()
            .route(
                "/",
                axum::routing::post(Self::configure).delete(Self::clear),
            )
            .with_state(self)
    }

    async fn configure(State(this): State<Self>) -> Result<Response, ContentAccessControllerError> {
        let cookies = this.configure_content_access.execute()?;
        Self::cookie_response(cookies, false)
    }

    async fn clear(State(this): State<Self>) -> Result<Response, ContentAccessControllerError> {
        let cookies = this.clear_content_access.execute()?;
        Self::cookie_response(cookies, true)
    }

    fn cookie_response(
        cookies: Vec<ContentAccessCookie>,
        removal: bool,
    ) -> Result<Response, ContentAccessControllerError> {
        let mut response = StatusCode::NO_CONTENT.into_response();

        for content_access_cookie in cookies {
            let mut cookie = Cookie::build((
                content_access_cookie.name().to_string(),
                content_access_cookie.value().to_string(),
            ))
            .path(content_access_cookie.path().to_string())
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();
            if let Some(domain) = content_access_cookie.domain() {
                cookie.set_domain(domain.to_string());
            }
            if removal {
                cookie.make_removal();
            }

            response.headers_mut().append(
                SET_COOKIE,
                cookie.to_string().parse().map_err(|e| {
                    ContentAccessControllerError(format!("failed to create cookie header: {e}"))
                })?,
            );
        }

        Ok(response)
    }
}

pub(crate) struct ContentAccessControllerError(String);

impl From<crate::domain::Error> for ContentAccessControllerError {
    fn from(error: crate::domain::Error) -> Self {
        Self(error.to_string())
    }
}

impl IntoResponse for ContentAccessControllerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(CONTENT_TYPE, "application/problem+json")],
            axum::Json(InternalServerErrorProblem::new(
                self.0,
                StatusCode::INTERNAL_SERVER_ERROR.as_u16().into(),
            )),
        )
            .into_response()
    }
}
