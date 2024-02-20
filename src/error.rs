// aca vamos a manejar los errores
// tenemos errores separados para diferente escenarios
use mongodb::bson;
use serde::Serialize;
use thiserror::Error;
use std::convert::Infallible;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error{
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Error al hacer una query: {0}")]
    MonogoQueryError(mongodb::error::Error),
    #[error("Valor no esperado: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("Id incorrecto: {0}")]
    InvalidIDError(String),
}


#[derive(Serialize)]
// aca defino como se vera el error
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

// esta es la funcion que luego uso en main
pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    // aca defino las variables que luego voy a modificar
    // segun sea el error;
    let code: StatusCode;
    let message: &str;


    // aca retorno segun cual sea el error
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>(){
        code = StatusCode::BAD_REQUEST;
        message = "Bad Request - Invalid body";
    }else if let Some(e) = err.find::<Error>(){
        match e {
            _ => {
                eprintln!("unhandler error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internarl server error";
            }
        }
    }else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed";
    }else {
        eprintln!("unhandler error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    }

    //aca cfeo el json para enviar en el error
    let json = reply::json(&ErrorResponse{
        message: message.into(),
    });

    // aca devuevlo el error, el json y el codigo
    Ok(Box::new(reply::with_status(json, code)))

}