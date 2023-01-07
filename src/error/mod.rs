pub mod handler;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ActixLabError {
    #[error("Tera template error. {0}")]
    TemplateError(#[from] tera::Error),
    // #[error("actix-identity error while login")]
    // IdentityLoginError,
    #[error("Other severe system error")]
    Other(#[from] anyhow::Error),
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
            ActixLabError::Other(_)
            // | ActixLabError::IdentityLoginError
            | ActixLabError::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::new(self.status_code())
    }
}