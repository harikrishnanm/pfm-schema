use actix_web::{web, Responder, HttpResponse};
use serde::Deserialize;
use validator::Validate;
use validator_derive::Validate;
use log::{info, error, debug, trace};
use diesel::{Queryable, Insertable, PgConnection, r2d2::{ConnectionManager, PooledConnection}};
use diesel::result::Error;
use serde::Serialize;
use crate::diesel::RunQueryDsl;
use crate::RequestContext;
use crate::schema::object_schema;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct JsonField {
    #[serde(alias = "fieldName")]
    pub field_name: String,
    #[serde(alias = "type")]
    pub field_type: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct SchemaBody {
    
    #[validate(length(min = 4, message = "Should be atleast 4 characters"))]
    #[validate(required)]
    pub namespace: Option<String>,


    #[validate(length(min = 8, message = "Should be atleast 8 characters"))]
    #[validate(required)]
    #[serde(alias = "objectName")]
    pub object_name: Option<String>,

    #[validate(length(min = 8, message = "Should be atleast 8 characters"))]
    #[validate(required)]
    #[serde(alias = "objectUrl")]
    pub object_url: Option<String>,

    pub fields: Option<Vec<JsonField>>
}

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "object_schema"]
pub struct ObjectSchema {
    pub id: i32,
    pub object_name: String,
    pub object_url: String,
    pub namespace: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "object_schema"]
pub struct NewObjectSchema {
    pub object_name: String,
    pub object_url: String,
    pub namespace: String
}

async fn store_schema(new_schema: &NewObjectSchema, conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Result<i32, Error> {
    info!("Storing schema object");

    match diesel::insert_into(object_schema::table)
        .values(new_schema)
        .get_result::<ObjectSchema>(conn) {
            Ok(result) => { 
                Ok(result.id)
            },
            Err(e) => {
                error!("Error saving schema {}", e.to_string());
                Err(e)
                /*match e {
                    Error::DatabaseError(kind, _) => {
                        match kind {
                            DatabaseErrorKind::UniqueViolation => HttpResponse::BadRequest().body(e.to_string()),
                            _ => HttpResponse::InternalServerError().finish()
                        }
                    },
                    _ => HttpResponse::InternalServerError().finish()

                }*/
            }
        }            
}
async fn process_fields(fields_option: &Option<Vec<JsonField>>, conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Result<i32, Error>{
    
    info!("Storing object fields");
    match fields_option {
        Some(fields) =>{
            debug!("Size of fields {}", fields.len());
            for field in fields {
                debug!("Processing field {:?}", field);
            }
            

        }
        None => {
            error!("No Field Definition");
        } 
    }

    Ok(2)
}

pub async fn register_schema(schema_request: web::Json<SchemaBody>, request_context: web::Data<RequestContext>) -> impl Responder {
    info!("In register_schema");
    trace!("Request json {:?}", schema_request) ;
    match schema_request.validate().err() {
        Some(err) => HttpResponse::BadRequest().json(err),
        None => {
            let name = &schema_request.object_name.as_ref();
            let url = &schema_request.object_url.as_ref();
            let namespace = &schema_request.namespace.as_ref();
            let new_object_schema = NewObjectSchema {
                object_name: name.unwrap().to_string(),
                object_url: url.unwrap().to_string(),
                namespace: namespace.unwrap().to_string()
            };

            let fields = &schema_request.fields;

            trace!("Object Schema {:?}", new_object_schema);

            //let fields = json!(&schema_request.fields.as_ref());
            trace!("Fields {:?}", fields);
            let connection_1: &PooledConnection<ConnectionManager<PgConnection>> = &request_context.db_pool.get().unwrap();
            let schema_store_future = store_schema(&new_object_schema, connection_1);
            //let schema_store_result = schema_store_future.await;

            let connection_2: &PooledConnection<ConnectionManager<PgConnection>> = &request_context.db_pool.get().unwrap();
            let process_field_future = process_fields(fields, connection_2);
            //let process_field_result = &process_field_future.await;

            futures::join!(schema_store_future, process_field_future);
            

            HttpResponse::Ok().finish()

            
        }
    }    
}
