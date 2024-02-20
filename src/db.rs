//Aca voy a definir los modelos de la DB
use crate::{error::Error::*, handlers::CooperativaRequest, Cooperativa, Result};
use std::env::var;
// para manejar el tiempo usamos chrono
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const COLL: &str = "cooperativas";

const ID: &str = "_id";
const NAME: &str = "name";
const NUM_SOCIOS: &str = "num_socios";
const ADDED_AT: &str = "added_at";
const SOCIOS: &str = "socios";


#[derive(Clone, Debug)]
pub struct DB{
    pub client: Client,
}

//Aca iniciamos la db
impl DB {
    pub async fn init() -> Result<Self> {
        dotenvy::dotenv().ok();
        let db_url = var("DB_URL").expect("Missing db url");
        let db_name = var("DB_NAME").expect("Missing db name");
        let mut client_options = ClientOptions::parse(&db_url).await?;

        client_options.app_name = Some(db_name.to_string());


        Ok(Self{
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_cooperativas(&self) -> Result<Vec<Cooperativa>> {
        let mut cursor = self
            .get_collection() // la defino en este documento
            .find(None, None)
            .await
            .map_err(MonogoQueryError)?;

        let mut result: Vec<Cooperativa> = Vec::new();
        while let Some(doc) =  cursor.next().await {
            result.push(self.doc_to_coop(&doc?)?); //definida en este doc
        }

        Ok(result)
    }

    // funciones de crear
    pub async fn create_cooperativa(&self, entry: &CooperativaRequest) -> Result<Vec<Cooperativa>> {  
        let doc = doc! {
            NAME: entry.name.clone(),
            NUM_SOCIOS: entry.num_socios as i32,
            ADDED_AT: Utc::now(),
            SOCIOS: entry.socios.clone(),
        };
        let inserted = self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MonogoQueryError)?;
        let filter = doc! {
            "_id": inserted.inserted_id
        };

        let mut cursor = self.get_collection()
            .find(filter, None)
            .await
            .map_err(MongoError)?;
        let mut result: Vec<Cooperativa> = Vec::new();

        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_coop(&doc?)?)
        }

        Ok(result)
    }

    //funcion para eliminar
    pub async fn delete_cooperativa(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid
        };
        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(MonogoQueryError)?;
        Ok(())
    }

    // funcion para editar
    pub async fn update_cooperativa(&self, id: &str, entry: &CooperativaRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid
        };

        let new_doc = doc! {
            NAME: entry.name.clone(),
            NUM_SOCIOS: entry.num_socios as i32,
            ADDED_AT: Utc::now(),
            SOCIOS: entry.socios.clone(),
        };

        self.get_collection()
        .update_one(query, new_doc, None)
        .await
        .map_err(MonogoQueryError)?;

        Ok(())
    }

    fn get_collection(&self) -> Collection{
        dotenvy::dotenv().ok();
        let db_name = var("DB_NAME").expect("Missing db name");
        self.client.database(&db_name).collection(COLL)
    }

    fn doc_to_coop(&self, doc: &Document) -> Result<Cooperativa> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let num_socios = doc.get_i32(NUM_SOCIOS)?;
        let added_at = doc.get_datetime(ADDED_AT)?;
        let socios = doc.get_array(SOCIOS)?;

        let cooperativa = Cooperativa{
            id: id.to_hex(),
            name: name.to_owned(),
            num_socios: num_socios as usize,
            added_at: *added_at,
            socios: socios
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };

        Ok(cooperativa)
    }
    
}