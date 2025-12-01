use axum::{Json, response::{Response,IntoResponse}, http::StatusCode};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;

use crate::utils::app_errors::AppError;



pub async fn get_cookie (jar: CookieJar) -> Result<Response, AppError> {
    let cookie = jar.get("auth_token");
    match cookie {
        Some(token) => {
            let token_val = token.value();
            Ok((StatusCode::OK, Json(json!({
                "auth_token": token_val
            }))).into_response())
        },
        None => {
          Err(AppError::NotFound(Some(String::from("Token not found")))) 
        }
    }
}