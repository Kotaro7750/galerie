use std::time::Duration;

use aliri::{jwa, jwt};
use aliri_clock::UnixTime;
use aliri_oauth2::{Authority, HasScope, Scope, scope::BasicClaimsWithScope};
use aliri_tower::Oauth2Authorizer;
use axum::body::Body;
use serde::Deserialize;
use tower::{Layer, Service};

use super::ConfigError;

const JWKS_REFRESH_INTERVAL: Duration = Duration::from_secs(60 * 60);

#[derive(Debug, Default, Deserialize)]
/// OAuth 2.0 access-token authorization settings.
pub(crate) struct AuthorizationConfig {
    #[serde(default)]
    enabled: bool,
    issuer: Option<String>,
    audience: Option<String>,
    jwks_url: Option<String>,
}

impl AuthorizationConfig {
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn issuer(&self) -> Option<&str> {
        self.issuer.as_deref()
    }

    pub(crate) fn audience(&self) -> Option<&str> {
        self.audience.as_deref()
    }

    pub(crate) fn jwks_url(&self) -> Option<&str> {
        self.jwks_url.as_deref()
    }

    pub(crate) async fn construct_auth_layer(
        &self,
    ) -> Result<
        Option<
            impl Layer<
                axum::routing::Route,
                Service: Service<
                    axum::http::Request<Body>,
                    Error = std::convert::Infallible,
                    Response: axum::response::IntoResponse + 'static,
                    Future: Send + 'static,
                > + Clone
                             + Send
                             + Sync
                             + 'static,
            > + Clone
            + use<>,
        >,
        ConfigError,
    > {
        if !self.enabled() {
            return Ok(None);
        }

        let validator = jwt::CoreValidator::default()
            .add_approved_algorithm(jwa::Algorithm::RS256)
            .add_allowed_audience(jwt::Audience::from(
                self.audience().expect("validated authorization audience"),
            ))
            .require_issuer(jwt::Issuer::from(
                self.issuer().expect("validated authorization issuer"),
            ));
        let authority = Authority::new_from_url(
            self.jwks_url()
                .expect("validated authorization JWKS URL")
                .to_owned(),
            validator,
        )
        .await
        .map_err(|error| {
            ConfigError::AuthorizationConstruction(format!("retrieving JWKS: {error}"))
        })?;
        authority.spawn_refresh(JWKS_REFRESH_INTERVAL);

        let authorizer = Oauth2Authorizer::new()
            .with_claims::<AccessTokenClaims>()
            .with_terse_error_handler::<Body>();

        Ok(Some(authorizer.jwt_layer::<Body>(authority)))
    }

    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if !self.enabled() {
            return Ok(());
        }

        for (name, value) in [
            ("issuer", self.issuer()),
            ("audience", self.audience()),
            ("jwks_url", self.jwks_url()),
        ] {
            if value.is_none_or(str::is_empty) {
                return Err(ConfigError::InvalidConfig(format!(
                    "authorization.{name} is required when authorization.enabled is true"
                )));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
/// Represents the claims in an OAuth 2.0 access token
pub(crate) struct AccessTokenClaims {
    #[serde(flatten)]
    standard: BasicClaimsWithScope,
    #[serde(default, rename = "token_use")]
    /// Cognito-specific claim indicating the type of token. Only `access` is accepted.
    _token_use: Option<TokenUse>,
}

#[derive(Clone, Debug, Deserialize)]
enum TokenUse {
    #[serde(rename = "access")]
    Access,
}

impl jwt::CoreClaims for AccessTokenClaims {
    fn nbf(&self) -> Option<UnixTime> {
        jwt::CoreClaims::nbf(&self.standard)
    }

    fn exp(&self) -> Option<UnixTime> {
        jwt::CoreClaims::exp(&self.standard)
    }

    fn aud(&self) -> &jwt::Audiences {
        jwt::CoreClaims::aud(&self.standard)
    }

    fn iss(&self) -> Option<&jwt::IssuerRef> {
        jwt::CoreClaims::iss(&self.standard)
    }

    fn sub(&self) -> Option<&jwt::SubjectRef> {
        jwt::CoreClaims::sub(&self.standard)
    }
}

impl HasScope for AccessTokenClaims {
    fn scope(&self) -> &Scope {
        HasScope::scope(&self.standard)
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessTokenClaims, AuthorizationConfig};

    const BASE_CLAIMS: &str = r#"{"iss":"https://issuer.example","aud":"https://api.galerie.example.com","sub":"user-123","exp":4102444800,"scope":"openid"}"#;

    #[test]
    fn disabled_authorization_does_not_require_provider_settings() {
        assert!(AuthorizationConfig::default().validate().is_ok());
    }

    #[test]
    fn enabled_authorization_requires_all_provider_settings() {
        let config = AuthorizationConfig {
            enabled: true,
            issuer: Some("https://issuer.example".to_string()),
            audience: Some("https://api.galerie.example.com".to_string()),
            jwks_url: None,
        };

        assert!(config.validate().is_err());

        let config = AuthorizationConfig {
            enabled: true,
            issuer: Some("https://issuer.example".to_string()),
            audience: None,
            jwks_url: Some("https://issuer.example/jwks.json".to_string()),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn accepts_cognito_access_tokens() {
        let claims = BASE_CLAIMS.replace('}', ",\"token_use\":\"access\"}");

        serde_json::from_str::<AccessTokenClaims>(&claims).unwrap();
    }

    #[test]
    fn rejects_cognito_id_tokens() {
        let claims = BASE_CLAIMS.replace('}', ",\"token_use\":\"id\"}");

        assert!(serde_json::from_str::<AccessTokenClaims>(&claims).is_err());
    }

    #[test]
    fn accepts_tokens_without_cognito_token_use() {
        assert!(serde_json::from_str::<AccessTokenClaims>(BASE_CLAIMS).is_ok());
    }
}
