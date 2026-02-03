use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use crate::infrastructure::auth::jwt::Claims;
use crate::usecase::auth::AuthUsecase;

#[derive(Clone)]
pub struct CurrentUser {
    pub id: String,
    pub email: String,
    pub role: String,
}

impl From<Claims> for CurrentUser {
    fn from(claims: Claims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
            role: claims.role,
        }
    }
}

#[derive(Serialize)]
pub struct AuthError {
    error: String,
    message: String,
}

pub async fn auth_middleware(
    State(auth_usecase): State<Arc<AuthUsecase>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token_from_header = auth_header.and_then(|h| h.strip_prefix("Bearer "));

    let token_from_query = request
        .uri()
        .query()
        .and_then(|q| q.split('&').find(|pair| pair.starts_with("token=")))
        .and_then(|pair| pair.strip_prefix("token="));

    let token = match (token_from_header, token_from_query) {
        (Some(t), _) => t,
        (None, Some(t)) => t,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: "unauthorized".to_string(),
                    message: "Missing or invalid authorization".to_string(),
                }),
            ));
        }
    };

    match auth_usecase.verify_token(token) {
        Ok(claims) => {
            // Verify user still exists in database
            match auth_usecase.get_user_by_id(&claims.sub).await {
                Ok(user) => {
                    let mut current_user = CurrentUser::from(claims);
                    current_user.role = user.role;
                    request.extensions_mut().insert(current_user);
                    Ok(next.run(request).await)
                }
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(AuthError {
                        error: "user_not_found".to_string(),
                        message: "User no longer exists. Please login again.".to_string(),
                    }),
                )),
            }
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "unauthorized".to_string(),
                message: "Invalid or expired token".to_string(),
            }),
        )),
    }
}
