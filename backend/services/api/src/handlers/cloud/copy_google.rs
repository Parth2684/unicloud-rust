use axum::{Extension, extract::Json, http::StatusCode, response::{IntoResponse, Response}};
use common::jwt_config::Claims;
use serde_json::json;

use crate::utils::app_errors::AppError;



struct CopyInputs {
    from_drive: String,
    from_file_id: String,
    to_drive: String,
    to_folder_id: String
}

pub async fn copy_file_or_folder(Extension(claims): Extension<Claims>, Json(payload): Json<CopyInputs>) -> Result<Response, AppError> {
    
    
    Ok((StatusCode::OK,
        axum::Json(json!({
        "message": "Task added successfully",
        
    }))).into_response())
}