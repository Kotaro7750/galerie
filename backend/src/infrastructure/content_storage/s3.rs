use std::num::NonZeroU64;

use async_trait::async_trait;
use aws_sdk_s3::{
    Client,
    config::http::HttpResponse,
    error::DisplayErrorContext,
    operation::list_objects_v2::{ListObjectsV2Error, ListObjectsV2Output},
};
use aws_smithy_async::future::pagination_stream::PaginationStream;

use crate::{
    domain::{Content, ContentId, Error, MediaType, tag::parse_metadata},
    port::ContentStorage,
};

#[derive(Debug)]
pub(crate) struct S3ContentStorage {
    client: Client,
    bucket_name: String,
    xmp_prefix: String,
    content_prefix: String,
    content_url_base: String,
    thumbnail_url_base: String,
}

impl S3ContentStorage {
    pub(crate) fn new(
        client: Client,
        bucket_name: String,
        xmp_prefix: String,
        content_prefix: String,
        content_url_base: String,
        thumbnail_url_base: String,
    ) -> Self {
        Self {
            client,
            bucket_name,
            xmp_prefix,
            content_prefix,
            content_url_base,
            thumbnail_url_base,
        }
    }
}

#[async_trait]
impl ContentStorage for S3ContentStorage {
    async fn scan_contents(
        &self,
        limit: NonZeroU64,
        cursor: Option<ContentId>,
    ) -> Result<(Vec<Content>, Option<ContentId>), Error> {
        let xmp_stream = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .prefix(&self.xmp_prefix);

        let content_stream = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .prefix(&self.content_prefix);

        let (xmp_stream, content_stream) = if let Some(cursor) = cursor {
            (
                xmp_stream.start_after(cursor.as_ref().to_string()),
                content_stream.start_after(cursor.as_ref().to_string()),
            )
        } else {
            (xmp_stream, content_stream)
        };
        let (mut xmp_stream, mut content_stream) = (
            xmp_stream.into_paginator().send(),
            content_stream.into_paginator().send(),
        );

        let mut xmp_stream = PaginatedObjectStream::new(&mut xmp_stream);
        let mut content_stream = PaginatedObjectStream::new(&mut content_stream);
        let mut xmp_object = xmp_stream.next_object().await?;
        let mut content_object = content_stream.next_object().await?;

        let mut contents = Vec::<Content>::with_capacity(limit.get() as usize + 1);

        while let (Some(xmp), Some(content)) = (&xmp_object, &content_object) {
            let Some(xmp_content_id) = xmp
                .key()
                .and_then(|key| extract_xmp_content_id(&self.xmp_prefix, key))
            else {
                xmp_object = xmp_stream.next_object().await?;
                continue;
            };
            let Some((content_content_id, media_type)) = content
                .key()
                .and_then(|key| extract_content_content_id(&self.content_prefix, key))
            else {
                content_object = content_stream.next_object().await?;
                continue;
            };

            match xmp_content_id.cmp(&content_content_id) {
                std::cmp::Ordering::Less => {
                    xmp_object = xmp_stream.next_object().await?;
                    continue;
                }
                std::cmp::Ordering::Greater => {
                    content_object = content_stream.next_object().await?;
                    continue;
                }
                std::cmp::Ordering::Equal => {}
            }

            if let Ok(metadata) = parse_metadata(&self.get_xmp_metadata(xmp_content_id).await?) {
                let content_url = format!(
                    "{}/{}.{}",
                    self.content_url_base,
                    xmp_content_id.as_ref(),
                    media_type.extension()
                )
                .parse::<url::Url>()
                .map_err(|e| Error::Internal(format!("Failed to construct content URL: {e}")))?;

                let thumbnail_url = format!(
                    "{}/{}.{}",
                    self.thumbnail_url_base,
                    xmp_content_id.as_ref(),
                    media_type.extension()
                )
                .parse::<url::Url>()
                .map_err(|e| Error::Internal(format!("Failed to construct content URL: {e}")))?;

                let content = Content::new(
                    xmp_content_id,
                    media_type,
                    content_url,
                    thumbnail_url,
                    metadata.parsed().clone(),
                    metadata.skipped().clone(),
                );

                contents.push(content);
                // This + 1 is to check for next cursot existence
                if contents.len() as u64 >= (limit.get() + 1) {
                    break;
                }
            }

            content_object = content_stream.next_object().await?;
            xmp_object = xmp_stream.next_object().await?;
        }

        // This + 1 is intentional. See above
        let next_cursor = if contents.len() as u64 >= (limit.get() + 1) {
            contents.pop();
            contents.last().map(|content| content.id())
        } else {
            None
        };

        Ok((contents, next_cursor))
    }
}

impl S3ContentStorage {
    async fn get_xmp_metadata(&self, content_id: ContentId) -> Result<String, Error> {
        let xmp_body = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(format!("{}/{}.xmp", self.xmp_prefix, content_id.as_ref()))
            .send()
            .await
            .map_err(|e| Error::Internal(format!("Failed to get XMP metadata: {}", e)))?
            .body
            .collect()
            .await
            .map_err(|e| Error::Internal(format!("Failed to read XMP metadata: {}", e)))?
            .into_bytes();

        String::from_utf8(xmp_body.to_vec())
            .map_err(|e| Error::Internal(format!("Failed to read XMP metadata: {}", e)))
    }
}

/// Extracts the `ContentId` and `MediaType` from the given S3 object key, if it matches the expected prefix and format.
/// This is for content files, which are expected to have the format `<prefix>/<uuid>.<extension>`.
fn extract_content_content_id(prefix: &str, key: &str) -> Option<(ContentId, MediaType)> {
    key.strip_prefix(prefix)
        .and_then(|name| name.strip_prefix('/'))
        .and_then(|name| name.rsplit_once("."))
        .and_then(|(stem, ext)| {
            if let (Some(id), Some(media_type)) = (
                uuid::Uuid::parse_str(stem).ok(),
                super::media_type_from_extension(ext),
            ) {
                Some((id, media_type))
            } else {
                None
            }
        })
        .and_then(|(id, media_type)| ContentId::try_from(id).ok().map(|id| (id, media_type)))
}

/// Extracts the `ContentId` from the given S3 object key, if it matches the expected prefix and format.
/// This is for XMP files, which are expected to have the format `<prefix>/<uuid>.xmp`.
fn extract_xmp_content_id(prefix: &str, key: &str) -> Option<ContentId> {
    key.strip_prefix(prefix)
        .and_then(|name| name.strip_prefix('/'))
        .and_then(|name| name.strip_suffix(".xmp"))
        .and_then(|stem| uuid::Uuid::parse_str(stem).ok())
        .and_then(|id| ContentId::try_from(id).ok())
}

#[derive(Debug)]
struct PaginatedObjectStream<'a> {
    objects: std::vec::IntoIter<aws_sdk_s3::types::Object>,
    pages: &'a mut PaginationStream<
        Result<
            ListObjectsV2Output,
            aws_smithy_runtime_api::client::result::SdkError<ListObjectsV2Error, HttpResponse>,
        >,
    >,
}

impl<'a> PaginatedObjectStream<'a> {
    fn new(
        pagination_stream: &'a mut PaginationStream<
            Result<
                ListObjectsV2Output,
                aws_smithy_runtime_api::client::result::SdkError<ListObjectsV2Error, HttpResponse>,
            >,
        >,
    ) -> Self {
        Self {
            objects: vec![].into_iter(),
            pages: pagination_stream,
        }
    }

    async fn next_object(&mut self) -> Result<Option<aws_sdk_s3::types::Object>, Error> {
        loop {
            // 現在のページ中にオブジェクトを発見したのでそれを返す
            if let Some(object) = self.objects.next() {
                return Ok(Some(object));
            }

            // 次のページを取得
            let Some(page) = self.pages.next().await else {
                return Ok(None);
            };

            self.objects = page
                .map_err(|e| {
                    Error::Internal(format!(
                        "Failed to list objects: {}",
                        DisplayErrorContext(&e)
                    ))
                })?
                .contents()
                .to_vec()
                .into_iter();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{num::NonZeroU64, str::FromStr};

    use aws_sdk_s3::{
        Client,
        operation::{get_object::GetObjectOutput, list_objects_v2::ListObjectsV2Output},
        primitives::ByteStream,
        types::Object,
    };
    use aws_smithy_mocks::{RuleMode, mock, mock_client};

    use super::S3ContentStorage;
    use crate::{domain::ContentId, port::ContentStorage};

    const BUCKET: &str = "test-bucket";
    const XMP_PREFIX: &str = "metadata";
    const CONTENT_PREFIX: &str = "contents";
    const XMP: &str = include_str!("../../../test-fixture/parse_xmp.xmp");

    fn storage(client: Client) -> S3ContentStorage {
        S3ContentStorage::new(
            client,
            BUCKET.to_string(),
            XMP_PREFIX.to_string(),
            CONTENT_PREFIX.to_string(),
            "https://example.com/content".to_string(),
            "https://example.com/thumbnail".to_string(),
        )
    }

    fn object(key: &str) -> Object {
        Object::builder().key(key).build()
    }

    #[tokio::test]
    async fn get_xmp_metadata_gets_the_xmp_object() {
        let content_id = ContentId::from_str("22222222-2222-4222-8222-222222222222").unwrap();
        let get_xmp = mock!(Client::get_object)
            .match_requests(|request| {
                request.bucket() == Some(BUCKET)
                    && request.key() == Some("metadata/22222222-2222-4222-8222-222222222222.xmp")
            })
            .then_output(|| {
                GetObjectOutput::builder()
                    .body(ByteStream::from_static(XMP.as_bytes()))
                    .build()
            });
        let client = mock_client!(aws_sdk_s3, RuleMode::MatchAny, [&get_xmp]);

        let metadata = storage(client).get_xmp_metadata(content_id).await.unwrap();

        assert_eq!(metadata, XMP);
        assert_eq!(get_xmp.num_calls(), 1);
    }

    #[tokio::test]
    async fn scan_contents_merge_joins_xmp_and_content_objects() {
        let list_xmp = mock!(Client::list_objects_v2)
            .match_requests(|request| {
                request.bucket() == Some(BUCKET) && request.prefix() == Some(XMP_PREFIX)
            })
            .then_compute_output(|request| {
                if request.continuation_token().is_none() {
                    ListObjectsV2Output::builder()
                        .contents(object("metadata/11111111-1111-4111-8111-111111111111.xmp"))
                        .contents(object("metadata/22222222-2222-4222-8222-222222222222.xmp"))
                        .is_truncated(true)
                        .next_continuation_token("xmp-page-2")
                        .build()
                } else {
                    assert_eq!(request.continuation_token(), Some("xmp-page-2"));
                    ListObjectsV2Output::builder()
                        .contents(object("metadata/44444444-4444-4444-8444-444444444444.xmp"))
                        .contents(object("metadata/55555555-5555-4555-8555-555555555555.txt"))
                        .contents(object("metadata/66666666-6666-4666-8666-666666666666.xmp"))
                        .contents(object("metadata/not-a-uuid.xmp"))
                        .is_truncated(false)
                        .build()
                }
            });
        let list_contents = mock!(Client::list_objects_v2)
            .match_requests(|request| {
                request.bucket() == Some(BUCKET) && request.prefix() == Some(CONTENT_PREFIX)
            })
            .then_compute_output(|request| {
                if request.continuation_token().is_none() {
                    ListObjectsV2Output::builder()
                        .contents(object("contents/22222222-2222-4222-8222-222222222222.avif"))
                        .contents(object("contents/33333333-3333-4333-8333-333333333333.avif"))
                        .is_truncated(true)
                        .next_continuation_token("content-page-2")
                        .build()
                } else {
                    assert_eq!(request.continuation_token(), Some("content-page-2"));
                    ListObjectsV2Output::builder()
                        .contents(object("contents/44444444-4444-4444-8444-444444444444.avif"))
                        .contents(object("contents/55555555-5555-4555-8555-555555555555.jpg"))
                        .contents(object("contents/66666666-6666-4666-8666-666666666666.avif"))
                        .contents(object("contents/not-a-uuid.avif"))
                        .is_truncated(false)
                        .build()
                }
            });
        let get_xmp = mock!(Client::get_object)
            .match_requests(|request| {
                request.bucket() == Some(BUCKET)
                    && matches!(
                        request.key(),
                        Some("metadata/22222222-2222-4222-8222-222222222222.xmp")
                            | Some("metadata/44444444-4444-4444-8444-444444444444.xmp")
                            | Some("metadata/66666666-6666-4666-8666-666666666666.xmp")
                    )
            })
            .then_output(|| {
                GetObjectOutput::builder()
                    .body(ByteStream::from_static(XMP.as_bytes()))
                    .build()
            });
        let client = mock_client!(
            aws_sdk_s3,
            RuleMode::MatchAny,
            [&list_xmp, &list_contents, &get_xmp]
        );

        let (contents, next_cursor) = storage(client)
            .scan_contents(NonZeroU64::new(10).unwrap(), None)
            .await
            .unwrap();

        let ids = contents
            .iter()
            .map(|content| content.id().as_ref().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            [
                "22222222-2222-4222-8222-222222222222",
                "44444444-4444-4444-8444-444444444444",
                "66666666-6666-4666-8666-666666666666",
            ]
        );
        assert!(next_cursor.is_none());
        assert_eq!(list_xmp.num_calls(), 2);
        assert_eq!(list_contents.num_calls(), 2);
        assert_eq!(get_xmp.num_calls(), 3);
    }
}
