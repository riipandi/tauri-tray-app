// Copyright 2023 Aris Ripandi <aris@example.com>
// SPDX-License-Identifier: Apache-2.0

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::fetcher::{fetch_api, ApiResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    id: u32,
    quote: String,
    author: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_single_quote(id: Option<u32>) -> Result<ApiResponse<Quote>, ApiResponse<String>> {
    let param_id = id.unwrap_or(1); // set default id
    let url = format!("https://dummyjson.com/quotes/{}", param_id);

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let method = reqwest::Method::GET;
    // let json_body = Some(vec![/* your JSON body bytes here */]);
    let body = None; // Optional body if needed

    match fetch_api(url, method, headers, body).await {
        Ok(response) => Ok(response),
        Err(err) => Err(ApiResponse {
            status_code: 500,
            message: err.to_string(),
            data: None,
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllQuotes {
    quotes: Vec<Quote>,
    total: u32,
    skip: u32,
    limit: u32,
}

#[tauri::command]
pub async fn get_quotes() -> Result<ApiResponse<AllQuotes>, ApiResponse<String>> {
    let url = String::from("https://dummyjson.com/quotes");

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let method = reqwest::Method::GET;
    let body = None;

    match fetch_api(url, method, headers, body).await {
        Ok(response) => Ok(response),
        Err(err) => Err(ApiResponse {
            status_code: 500,
            message: err.to_string(),
            data: None,
        }),
    }
}
