use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Unauthorised(Option<String>),
    NotFound(Option<String>),
    Internal(Option<String>),
    Forbidden(Option<String>),
    UnprocessableEntry(Option<String>),
    BadGateway(Option<String>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Internal(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Internal Server Error"));
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
            AppError::NotFound(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Not Found"));
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
            AppError::Unauthorised(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Unauthorised"));
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
            AppError::Forbidden(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Forbidden"));
                (
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
            AppError::UnprocessableEntry(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Unprocessable Entry"));
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
            AppError::BadGateway(msg) => {
                let message = msg.unwrap_or_else(|| String::from("Bad Gateway"));
                (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({
                        "message": message
                    })),
                )
                    .into_response()
            }
        }
    }
}
