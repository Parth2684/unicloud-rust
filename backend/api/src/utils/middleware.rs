use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;
use chrono::Utc;

use crate::{
    handlers::auth::jwt_config::{Claims, decode_jwt},
    utils::app_errors::AppError,
};

pub async fn auth_middleware(
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let jwt = jar.get("auth_token");
    let jwt = match jwt {
        Some(token) => token,
        None => {
            return Err(AppError::Unauthorised(Some(String::from(
                "Authorisation Token not found",
            ))));
        }
    };
    let token = jwt.to_string();
    let claims = decode_jwt(&token);
    match claims {
        Ok(claim) => {
            let current_time = Utc::now();
            let time = current_time.timestamp();
            if time > claim.exp {
                return Err(AppError::Unauthorised(Some(String::from("Token expired"))));
            } else {
                request.extensions_mut().insert::<Claims>(claim);
                let response = next.run(request).await;
                Ok(response)
            }
        }
        Err(err) => Err(AppError::Unauthorised(Some(err.to_string()))),
    }
}
