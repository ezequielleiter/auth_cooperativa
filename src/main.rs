// Crono lo voy a necesitar para las fechas
use chrono::prelude::*;
use db::DB;
use serde::{Serialize, Deserialize};
// :Infallible nos ayuda a evitar errores
use std::convert::Infallible;


//A super-easy, composable, web server framework for warp speeds.
// Filter es para crear las rutas de la API
// Rejection nos ayuda a manejar los errores de las rutas
use warp::{Filter, Rejection};

// defino el tipo de error, se usa el char de erro
type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handlers;

// Es importante empezar desde los struct para definir
// las respuestas de la DB
// para poder trabajar con la DB necesito Serealize
#[derive(Serialize, Deserialize, Debug)]
pub struct Cooperativa {
    pub id: String,
    pub name: String,
    pub num_socios: usize,
    pub added_at: DateTime<Utc>,
    pub socios: Vec<String>,
}


// esta funcion me va a ayudar a armar las rutas
// para armar una fn asyn necesito tokio
#[tokio::main]
async fn main() -> Result<()> {
    // me conecto a la base de datos
    let db = DB::init().await?;
    //creo la api de las coopertaivas
    let cooperativa = warp::path("cooperativa");
    let cooperativa_routes = cooperativa
        .and(warp::post()) // POST REQ
        .and(warp::body::json())
        .and(with_db(db.clone())) // aca llamo a la db
        .and_then(handlers::create_cooperativa_handler) // aca uso los handler
        .or(cooperativa 
            .and(warp::put())// PUT REQ
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handlers::edit_cooperativa_handler))
        .or(cooperativa
            .and(warp::delete())// DEL REQ
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handlers::delet_cooperativa_handler))
        .or(cooperativa
            .and(warp::get())// GET REQ
            .and(with_db(db.clone()))
            .and_then(handlers::get_cooperativas));
        
        // con esta parte cacheo los errores
        // y los manejo con el handler erro
        let routes = cooperativa_routes.recover(error::handle_rejection);
        //aca creo el server
        println!("Corriendo en el puerto 8080");
        warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
       
        Ok(())
}


// Esta línea define una función llamada with_db que toma un parámetro db del tipo DB. Esta función devuelve un tipo que implementa el trait Filter con las siguientes características:
// Extract = (DB,): Indica que el tipo de dato extraído es una tupla que contiene un solo elemento de tipo DB.
// Error = Infallible: Indica que esta función nunca fallará.
// Además, el tipo devuelto por esta función también implementa el trait Clone, lo que significa que los valores de DB pueden ser clonados


// la función with_db toma una instancia de la base de datos DB 
// y devuelve un filtro que extrae esa instancia de la base de datos de cada solicitud entrante
fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
    // Esta línea utiliza la función any() del módulo warp,
    // que crea un filtro que coincide con cualquier solicitud. 
    // El método map() se utiliza para transformar la solicitud entrante. 
    // En este caso, toma un cierre (move || { db.clone() }) que clona el valor db y lo mueve al cierre. 
    // Esto significa que el cierre posee la propiedad del valor db, permitiendo que sea capturado por el contexto.

}