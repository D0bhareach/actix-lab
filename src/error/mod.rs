pub mod handler;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum ActixLabError {
    #[error("Tera template error. {0}")]
    TemplateError(#[from] tera::Error),
    #[error("Database error")]
    DbError(#[from] rusqlite::Error),
    #[error(transparent)]
    R2D2Error(#[from] r2d2::Error),
    #[error(transparent)]
    BlockingError(#[from] actix_web::error::BlockingError),
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
            Self::DbError(_)
            | Self::R2D2Error(_)
            | Self::Other(_)
            | Self::BlockingError(_)
            // | ActixLabError::IdentityLoginError
            | Self::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::new(self.status_code())
    }
}