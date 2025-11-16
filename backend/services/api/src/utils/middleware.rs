use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

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
        Some(token) => token.value(),
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
            request.extensions_mut().insert::<Claims>(claim);
            let response = next.run(request).await;
            Ok(response)
        }
        Err(err) => Err(AppError::Unauthorised(Some(err.to_string()))),
    }
}
