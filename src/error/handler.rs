use actix_web::{
    dev::{self, ServiceResponse}, 
    Result, HttpResponseBuilder,
    middleware::ErrorHandlerResponse,
    http::{StatusCode},
};
use lazy_static::lazy_static;
use std::sync::Arc;
use tera::Tera;

lazy_static!{
    pub static ref TERA: Arc<Tera> = Arc::new(
    tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap()
    );
}

// this function is not async fn? When I tried to change to async it threw.
// Not shure do I need to have TERA in Arc?
// I couldn't get tera instance from res.request().app_data(), that is why I have created
// tera in lazy_static.
pub fn not_found_handler<B>(res: dev::ServiceResponse<B>) -> 
Result<ErrorHandlerResponse<B>> {
        // default response in case if tera won't work.
        let mut response_string = String::from(r#"
            <body>
                <p>This is responce from raw string</p>
            </body>
        "#);

        let new_req = res.request().clone();
        let tera = TERA.clone();
            if let Ok(tmpl) = tera.render("404.html", &tera::Context::new()){
                response_string = tmpl;
            } else {
                // TODO: will need to log in logging file.
                log::error!("Can not render tera template from file")
            }
        let new_res = HttpResponseBuilder::new(StatusCode::OK).body(response_string);

        let res = ServiceResponse::new(new_req, new_res.map_into_right_body());

        Ok(ErrorHandlerResponse::Response(res))
    

}

pub fn internal_error_handler<B>(res: dev::ServiceResponse<B>) -> 
Result<ErrorHandlerResponse<B>> {
        // default response in case if tera won't work.
        let mut response_string = String::from(r#"
            <body>
                <p>Severe Internal Server Error.</p>
            </body>
        "#);

        let new_req = res.request().clone();
        let tera = TERA.clone();
            if let Ok(tmpl) = tera.render("500.html", &tera::Context::new()){
                response_string = tmpl;
            } else {
                // TODO: will need to log in logging file.
                // Do I really need this else block?
                log::error!("Can not render tera template from file")
            }
        let new_res = HttpResponseBuilder::new(StatusCode::OK).body(response_string);

        let res = ServiceResponse::new(new_req, new_res.map_into_right_body());

        Ok(ErrorHandlerResponse::Response(res))
    

}