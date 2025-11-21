use std::sync::Arc;

use axum::{extract::{State, WebSocketUpgrade}, response::IntoResponse};
use common::jwt_config::decode_jwt;
use cookie::Cookie;
use hyper::HeaderMap;

use crate::AppState;

async fn ws_handler (ws: WebSocketUpgrade, headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let cookies = headers.get("auth_token");
    let cookies = match cookies {
        Some(cookie) => match cookie.to_str() {
            Ok(str) => str.to_owned(),
            Err(err) => return "Error parsing cookie in websockets".into_response()
        },
        None => return "Cookie not found".into_response()
    };
    let mut token = String::from("");
    for part in cookies.split(';'){
        if let Ok(c) = Cookie::parse(part.trim().to_owned()) {
            if c.name() == "auth_token" {
                token = Some(c.value());
            }
        }
    }
    let claims = decode_jwt(token);
    
}