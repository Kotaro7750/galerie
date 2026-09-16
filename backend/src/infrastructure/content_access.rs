use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine, engine::general_purpose::STANDARD};
use rsa::{Pkcs1v15Sign, RsaPrivateKey};
use sha1::{Digest, Sha1};

use crate::domain::{Error, content_access::ContentAccessCookie};
use crate::port::ContentAccessConfigurator;

const POLICY_COOKIE_NAME: &str = "CloudFront-Policy";
const SIGNATURE_COOKIE_NAME: &str = "CloudFront-Signature";
const KEY_PAIR_ID_COOKIE_NAME: &str = "CloudFront-Key-Pair-Id";
const COOKIE_NAMES: [&str; 3] = [
    POLICY_COOKIE_NAME,
    SIGNATURE_COOKIE_NAME,
    KEY_PAIR_ID_COOKIE_NAME,
];

/// A no-op implementation of `ContentAccessConfigurator` that does not set any cookies.
pub(crate) struct NopContentAccessConfigurator;

impl ContentAccessConfigurator for NopContentAccessConfigurator {
    fn configure(&self) -> Result<Vec<ContentAccessCookie>, Error> {
        Ok(Vec::new())
    }

    fn clear(&self) -> Result<Vec<ContentAccessCookie>, Error> {
        Ok(Vec::new())
    }
}

/// A cloudfront implementation of `ContentAccessConfigurator` that generates signed cookies for accessing content.
pub(crate) struct CloudFrontContentAccessConfigurator {
    key_pair_id: String,
    private_key: RsaPrivateKey,
    resource_url: String,
    validity_seconds: u64,
    cookie_domain: Option<String>,
    cookie_path: String,
}

impl CloudFrontContentAccessConfigurator {
    pub(crate) fn new(
        key_pair_id: String,
        private_key: RsaPrivateKey,
        resource_url: String,
        validity_seconds: u64,
        cookie_domain: Option<String>,
        cookie_path: String,
    ) -> Self {
        Self {
            key_pair_id,
            private_key,
            resource_url,
            validity_seconds,
            cookie_domain,
            cookie_path,
        }
    }
}

impl ContentAccessConfigurator for CloudFrontContentAccessConfigurator {
    fn configure(&self) -> Result<Vec<ContentAccessCookie>, Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::Internal(format!("system clock is before Unix epoch: {e}")))?;
        let expires_at = now
            .as_secs()
            .checked_add(self.validity_seconds)
            .ok_or_else(|| Error::Internal("content access expiration overflow".to_string()))?;

        let policy = serde_json::json!({"Statement": [{
            "Resource": self.resource_url,
            "Condition": {"DateLessThan": {"AWS:EpochTime": expires_at}}
        }]})
        .to_string();
        let signature = self
            .private_key
            .sign(
                Pkcs1v15Sign::new::<Sha1>(),
                &Sha1::digest(policy.as_bytes()),
            )
            .map_err(|e| Error::Internal(format!("failed to sign content access policy: {e}")))?;
        let encode = |value: &[u8]| {
            STANDARD
                .encode(value)
                .replace('+', "-")
                .replace('=', "_")
                .replace('/', "~")
        };
        let values = [
            (POLICY_COOKIE_NAME, encode(policy.as_bytes())),
            (SIGNATURE_COOKIE_NAME, encode(&signature)),
            (KEY_PAIR_ID_COOKIE_NAME, self.key_pair_id.clone()),
        ];

        Ok(values
            .into_iter()
            .map(|(name, value)| {
                ContentAccessCookie::new(
                    name.to_string(),
                    value,
                    self.cookie_domain.clone(),
                    self.cookie_path.clone(),
                )
            })
            .collect())
    }

    fn clear(&self) -> Result<Vec<ContentAccessCookie>, Error> {
        Ok(COOKIE_NAMES
            .into_iter()
            .map(|name| {
                ContentAccessCookie::new(
                    name.to_string(),
                    String::new(),
                    self.cookie_domain.clone(),
                    self.cookie_path.clone(),
                )
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use rsa::pkcs8::DecodePrivateKey;

    use super::*;

    #[test]
    fn default_configurator_does_not_set_cookies() {
        let cookies = NopContentAccessConfigurator.configure().unwrap();
        assert!(cookies.is_empty());
        let cleared_cookies = NopContentAccessConfigurator.clear().unwrap();
        assert!(cleared_cookies.is_empty());
    }

    #[test]
    fn cloudfront_configurator_creates_opaque_cookies() {
        let configurator = CloudFrontContentAccessConfigurator::new(
            "key-pair-id".to_string(),
            RsaPrivateKey::from_pkcs8_pem(include_str!(
                "../../test-fixture/cloudfront_test_private_key.pem"
            ))
            .unwrap(),
            "https://example.cloudfront.net/*".to_string(),
            300,
            Some("example.net".to_string()),
            "/".to_string(),
        );

        let cookies = configurator.configure().unwrap();
        assert_eq!(cookies.len(), 3);
        assert!(cookies.iter().all(|cookie| !cookie.value().is_empty()));
        assert!(
            cookies
                .iter()
                .all(|cookie| cookie.domain() == Some("example.net") && cookie.path() == "/")
        );

        let cleared_cookies = configurator.clear().unwrap();
        assert_eq!(cleared_cookies.len(), 3);
        assert!(
            cleared_cookies
                .iter()
                .all(|cookie| cookie.value().is_empty())
        );
    }
}
