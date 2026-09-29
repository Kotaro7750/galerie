use axum::{Json, extract::State};
use galerie_api::models::TagSchema as ApiTagSchema;

use crate::domain::tag_schema::TagSchema;

#[derive(Clone)]
pub(crate) struct TagSchemaController {
    schema: TagSchema,
}

impl TagSchemaController {
    pub(crate) fn new(schema: TagSchema) -> Self {
        Self { schema }
    }

    pub(crate) fn router(self) -> axum::Router {
        axum::Router::new()
            .route("/", axum::routing::get(Self::get_tag_schema))
            .with_state(self)
    }

    async fn get_tag_schema(State(this): State<Self>) -> Json<ApiTagSchema> {
        Json((&this.schema).into())
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use tower::ServiceExt;

    use super::*;

    async fn get_schema(schema: TagSchema) -> serde_json::Value {
        let response = Router::new()
            .nest(
                "/api/v0/tag-schema",
                TagSchemaController::new(schema).router(),
            )
            .oneshot(
                Request::builder()
                    .uri("/api/v0/tag-schema")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn returns_configured_schema_with_definitions() {
        let schema: TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\nrequired:\n  subjects:\n    type: text_set\noptional:\n  category:\n    type: text\n    allowed_values: [landscape, abstract]\n",
        )
        .unwrap();

        let response = get_schema(schema).await;
        assert_eq!(response["version"], "0");
        assert_eq!(response["allowAdditionalTags"], false);
        assert_eq!(
            response["required"],
            json!([{"key": "subjects", "type": "textSet"}])
        );
        assert_eq!(response["optional"].as_array().unwrap().len(), 1);
        assert_eq!(response["optional"][0]["key"], "category");
        assert_eq!(response["optional"][0]["type"], "text");
        assert_eq!(
            response["optional"][0]["allowedValues"],
            json!(["abstract", "landscape"])
        );
    }
}
