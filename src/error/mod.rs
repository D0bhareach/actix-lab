pub mod handler;
use actix_web::{ResponseError,};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ActixLabError {
    #[error("actix_lab template error")]
    TemplateError(#[from]tera::Error),
}
use actix_web::http::StatusCode; 

impl ResponseError for ActixLabError {
    fn status_code(&self) -> StatusCode {
        match self {
              /*
              ActixLabErrr::ValidationError(_) => StatusCode::BAD_REQUEST,
              ActixLabError::DatabaseError(_)
            | ActixLabError::StoreTokenError(_)
            */
            ActixLabError::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}