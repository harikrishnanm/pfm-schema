use diesel::{Queryable, Insertable};
use serde::Serialize;


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "object_schema"]
pub struct ObjectSchema {
    pub object_name: String,
    pub object_url: String
}