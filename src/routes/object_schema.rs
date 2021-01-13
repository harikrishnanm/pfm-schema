use crate::diesel::RunQueryDsl;
use crate::schema::field_schema;
use crate::schema::object_schema;
use crate::RequestContext;
use actix_web::{web, HttpResponse, Responder};
use bigdecimal::BigDecimal;
use diesel::{result::Error, types::Int4};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    Insertable, PgConnection, Queryable,
};
use log::{debug, error, info, trace};
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
use validator_derive::Validate;

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

    pub fields: Option<Vec<JsonField>>,
}

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "object_schema"]
pub struct ObjectSchema {
    pub id: i32,
    pub object_name: String,
    pub object_url: String,
    pub namespace: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "object_schema"]
pub struct NewObjectSchema {
    pub object_name: String,
    pub object_url: String,
    pub namespace: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "field_schema"]
pub struct FieldSchema {
    pub field_name: Int4,
    pub is_array: bool,
    pub is_required: bool,
    pub object_id: Int4,
    pub parent: Int4,
    pub pattern: Option<String>,
    pub size: Option<Int4>,
    pub field_type: String,
}

#[derive(Serialize)]
pub struct ObjectSchemaResponse {
    pub object_schema_id: Option<i32>,
    pub errors: Option<String>,
    pub field_count: Option<i32>,
}

fn store_schema(
    new_schema: &NewObjectSchema,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i32, Error> {
    info!("Storing schema object");

    match diesel::insert_into(object_schema::table)
        .values(new_schema)
        .get_result::<ObjectSchema>(conn)
    {
        Ok(result) => Ok(result.id),
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

fn process_fields(
    object_id: i32,
    parent_id: Option<i32>,
    fields_option: &Option<Vec<JsonField>>,
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<i32, Error> {
    info!("Storing object fields");
    let mut count: i32 = 0;
    match fields_option {
        Some(fields) => {
            debug!("Size of fields {}", fields.len());
            for field in fields {
                debug!("Processing field {:?}", field);

                let field_type = &field.field_type;
                if field_type == "OBJECT" {
                    debug!("Object Field {}", &field.field_name);
                } else {
                }
            }
        }
        None => {
            error!("No Field Definition");
        }
    }

    Ok(count)
}

pub async fn register_schema(
    schema_request: web::Json<SchemaBody>,
    request_context: web::Data<RequestContext>,
) -> impl Responder {
    info!("In register_schema");
    trace!("Request json {:?}", schema_request);
    match schema_request.validate().err() {
        Some(err) => HttpResponse::BadRequest().json(err),
        None => {
            let name = &schema_request.object_name.as_ref();
            let url = &schema_request.object_url.as_ref();
            let namespace = &schema_request.namespace.as_ref();
            let new_object_schema = NewObjectSchema {
                object_name: name.unwrap().to_string(),
                object_url: url.unwrap().to_string(),
                namespace: namespace.unwrap().to_string(),
            };

            let fields = &schema_request.fields;

            trace!("Object Schema {:?}", new_object_schema);

            //let fields = json!(&schema_request.fields.as_ref());
            trace!("Fields {:?}", fields);
            let connection = &request_context.db_pool.get().unwrap();
            match store_schema(&new_object_schema, connection.clone()) {
                Ok(id) => {
                    debug!("Saved schema header. Now processing fields");
                    trace!("Saved schema id {}", id);
                    match process_fields(id, Some(0), fields, connection.clone()) {
                        Ok(count) => HttpResponse::Created().json(ObjectSchemaResponse {
                            object_schema_id: Some(id),
                            errors: None,
                            field_count: Some(count),
                        }),
                        Err(e) => HttpResponse::BadRequest().json(ObjectSchemaResponse {
                            object_schema_id: None,
                            field_count: None,
                            errors: Some(e.to_string()),
                        }),
                    }
                }
                Err(e) => HttpResponse::BadRequest().json(ObjectSchemaResponse {
                    object_schema_id: None,
                    field_count: None,
                    errors: Some(e.to_string()),
                }),
            }
        }
    }
}
