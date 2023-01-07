use actix_web::{
    dev::{self, ServiceResponse}, 
    Result, HttpResponseBuilder,
    middleware::ErrorHandlerResponse,
    http::{StatusCode}, HttpRequest,
};
use lazy_static::lazy_static;
use std::sync::Arc;
use tera::Tera;

use super::ActixLabError;

lazy_static!{
    pub static ref TERA: Arc<Tera> = Arc::new(
    tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap()
    );
}

        const FILE_404: &str = "404.html";
        const FILE_500: &str = "500.html";
// this function is not async fn? When I tried to change to async it threw.
// Not shure do I need to have TERA in Arc?
// I couldn't get tera instance from res.request().app_data(), that is why I have created
// tera in lazy_static.

fn template_as_string(file: &str) -> Result<String, ActixLabError> {
        let tera = TERA.clone();
        #[allow(clippy::redundant_closure)]
        tera.render(file, &tera::Context::new())
        .map_err(|e| ActixLabError::TemplateError(e))

}

fn error_handler_response<B>(req: HttpRequest, res_string: String) -> ErrorHandlerResponse<B>{
        let new_res = HttpResponseBuilder::new(StatusCode::OK).body(res_string);
        let res = ServiceResponse::new(req, new_res.map_into_right_body());
        ErrorHandlerResponse::Response(res)

}

pub fn not_found_handler<B>(res: dev::ServiceResponse<B>) -> 
Result<ErrorHandlerResponse<B>> {
        let response_string = template_as_string(FILE_404)
        .unwrap_or_else(|e| {
                tracing::error!("{}", e);
            String::from(r#"
                        <body>
                            <p>This is responce from raw string. </p>
                        </body>
                    "#)
        });

        Ok(error_handler_response(res.request().clone(), response_string))
}

pub fn internal_error_handler<B>(res: dev::ServiceResponse<B>) -> 
Result<ErrorHandlerResponse<B>> {
        let response_string = template_as_string(FILE_500)
        .unwrap_or_else(|e| {
                tracing::error!("{}", e);
            String::from(r#"
                        <body>
                            <p>Severe Internal Server Error.</p>
                        </body>
                    "#)
        });

        Ok(error_handler_response(res.request().clone(), response_string))
}