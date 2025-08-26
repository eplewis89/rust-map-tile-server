use crate::{
    response::GenericResponse,
    WebResult,
};

use warp::{reply::json, Reply};

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}