// Copyright 2023 Aris Ripandi <aris@example.com>
// SPDX-License-Identifier: Apache-2.0

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status_code: u16,
    pub message: String,
    pub data: Option<T>,
}

pub async fn fetch_api<T>(
    url: String,
    method: reqwest::Method,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
) -> Result<ApiResponse<T>, reqwest::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let client = reqwest::Client::new();
    let mut request = client.request(method, url).headers(headers);

    if let Some(body) = body {
        request = request.body(body);
    }

    let response = request.send().await?;
    let resp_status = response.status();

    if resp_status.is_success() {
        let data: T = response.json().await?;
        let api_response = ApiResponse {
            status_code: resp_status.as_u16(),
            message: "Success".to_string(), // Set a default success message
            data: Some(data),
        };

        Ok(api_response)
    } else {
        let error_response: ApiResponse<String> = response.json().await?; // Assuming error response includes a "message" field
        let api_response = ApiResponse {
            status_code: resp_status.as_u16(),
            message: error_response.message,
            data: None,
        };

        Ok(api_response)
    }
}
