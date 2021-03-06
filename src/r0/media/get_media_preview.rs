//! [GET /_matrix/media/r0/preview_url](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-media-r0-preview-url)

use std::time::SystemTime;

use ruma_api::ruma_api;
use serde_json::Value as JsonValue;

ruma_api! {
    metadata {
        description: "Get a preview for a URL.",
        name: "get_media_preview",
        method: GET,
        path: "/_matrix/media/r0/preview_url",
        rate_limited: true,
        requires_authentication: true,
    }

    request {
        /// URL to get a preview of.
        #[ruma_api(query)]
        pub url: String,

        /// Preferred point in time (in milliseconds) to return a preview for.
        #[ruma_api(query)]
        #[serde(with = "ruma_serde::time::ms_since_unix_epoch")]
        pub ts: SystemTime,
    }

    response {
        /// OpenGraph-like data for the URL.
        ///
        /// Differences from OpenGraph: the image size in bytes is added to the `matrix:image:size`
        /// field, and `og:image` returns the MXC URI to the image, if any.
        #[ruma_api(body)]
        pub data: Option<JsonValue>,
    }

    error: crate::Error
}
