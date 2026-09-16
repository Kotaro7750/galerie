# Backend

## Common

The container image defines the following environment variables.

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_LISTEN_ADDRESS` | Yes | `0.0.0.0` | Address on which the API server listens. |
| `GALERIE_LISTEN_PORT` | Yes | `3000` | Port on which the API server listens. |

## Content Storage

Content storage has no image default. Set `GALERIE_CONTENT_STORAGE__MODE` and the variables required for its selected variant.

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_CONTENT_STORAGE__MODE` | Yes | - | Selects the `FileSystem` or `S3` variant. |

### FileSystem

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_CONTENT_STORAGE__FILE_SYSTEM__CONTENT_ROOT_PATH` | Yes | - | Path to the directory containing content and metadata files. Mount this path when running in a container. |
| `GALERIE_CONTENT_STORAGE__FILE_SYSTEM__CONTENT_URL_BASE` | Yes | - | Base URL used to build content URLs returned by the API. |
| `GALERIE_CONTENT_STORAGE__FILE_SYSTEM__THUMBNAIL_URL_BASE` | Yes | - | Base URL used to build thumbnail URLs returned by the API. |

### S3

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_CONTENT_STORAGE__S3__REGION` | Yes | - | AWS region containing the S3 bucket. |
| `GALERIE_CONTENT_STORAGE__S3__BUCKET_NAME` | Yes | - | Name of the S3 bucket containing the content. |
| `GALERIE_CONTENT_STORAGE__S3__XMP_PREFIX` | Yes | - | Object key prefix for XMP metadata files. |
| `GALERIE_CONTENT_STORAGE__S3__CONTENT_PREFIX` | Yes | - | Object key prefix for content files. |
| `GALERIE_CONTENT_STORAGE__S3__CONTENT_URL_BASE` | Yes | - | Base URL used to build content URLs returned by the API. |
| `GALERIE_CONTENT_STORAGE__S3__THUMBNAIL_URL_BASE` | Yes | - | Base URL used to build thumbnail URLs returned by the API. |

AWS credentials are resolved through the AWS SDK credential provider chain.

## Content Access

Content access control is disabled by default with `GALERIE_CONTENT_ACCESS__MODE=Nop`.

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_CONTENT_ACCESS__MODE` | Yes | `Nop` | Selects `Nop` or `CloudFront`; the latter enables CloudFront signed cookies. |

### CloudFront

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__KEY_PAIR_ID` | Yes | - | CloudFront key pair ID associated with the signing key. |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__PRIVATE_KEY` | Yes | - | PKCS#8 PEM private key used to sign cookies. |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__RESOURCE_URL` | Yes | - | CloudFront resource URL covered by the signed-cookie policy. |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__VALIDITY_SECONDS` | Yes | - | Number of seconds for which generated cookies remain valid. |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__COOKIE_DOMAIN` | No | - | Optional domain attribute applied to generated cookies. |
| `GALERIE_CONTENT_ACCESS__CLOUD_FRONT__COOKIE_PATH` | Yes | `/` | Path attribute applied to generated cookies. |
