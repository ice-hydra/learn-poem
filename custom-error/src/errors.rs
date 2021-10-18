use serde::Serialize;
use poem::{web::Json, IntoResponse};

#[derive(Serialize)]
pub struct AppError {
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> poem::Response {
        Json(self).with_status(poem::http::StatusCode::BAD_REQUEST)
            .into_response()
}}
