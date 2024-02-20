// tomo el type de webresult creado en main
use crate::{db::DB, WebResult};
use serde::{Serialize, Deserialize};
use warp::{reject, Reply, reply::json, http::StatusCode};

// aca voy a manejar todas las REQ a la api

// defino como se van a ver las REQ
#[derive(Serialize, Deserialize, Debug)]
pub struct CooperativaRequest {
    // pub id: String,
    pub name: String,
    pub num_socios: usize,
    // pub added_at: DateTime<Utc>,
    pub socios: Vec<String>,
}


pub async fn create_cooperativa_handler(body: CooperativaRequest, db: DB) -> WebResult<impl Reply>{
    let result = db.create_cooperativa(&body).await.map_err(|e| reject::custom(e))?;
    Ok(json(&result[0]))
}

pub async fn edit_cooperativa_handler(id: String, body: CooperativaRequest, db: DB) -> WebResult<impl Reply>{
    db.update_cooperativa(&id, &body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delet_cooperativa_handler(id: String, db: DB) -> WebResult<impl Reply>{
    db.delete_cooperativa(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn get_cooperativas(db: DB) -> WebResult<impl Reply>{
 //con el map erro cacheo los error
 let cooperativas = db.fetch_cooperativas().await.map_err(|e| reject::custom(e))?;

 // devuelvo el json de las cooperativas
 Ok(json(&cooperativas))
}
