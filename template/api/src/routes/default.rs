use crate::{{crate_name}};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use anyhow::Context;
use ::{{crate_name}}::Object;

impl ResponseError for {{crate_name}}::{{project-name | capitalize}}Error {
    fn status_code(&self) -> StatusCode {
        match self {
            {{crate_name}}::{{project-name | capitalize}}Error::InvalidDataError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            {{crate_name}}::{{project-name | capitalize}}Error::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[tracing::instrument(level = "debug")]
pub async fn hello() -> Result<HttpResponse, {{crate_name}}::{{project-name | capitalize}}Error> {
    let object: Object = {{crate_name}}::build_object()?;

    Ok(HttpResponse::Ok()
       .content_type("application/json")
       .body(serde_json::to_string(&object).context("Cannot serialize object")?))
}


#[tracing::instrument(level = "debug")]
pub async fn option_wildcard() -> HttpResponse {
    HttpResponse::Ok().finish()
}
