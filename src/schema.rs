table! {
    field_schema (id) {
        id -> Int4,
        field_name -> Varchar,
        is_array -> Bool,
        is_required -> Bool,
        object_id -> Nullable<Int4>,
        parent -> Nullable<Int4>,
        pattern -> Nullable<Varchar>,
        size -> Nullable<Int4>,
        field_type -> Nullable<Varchar>,
    }
}

table! {
    object_schema (id) {
        id -> Int4,
        namespace -> Varchar,
        object_name -> Varchar,
        object_url -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(field_schema, object_schema,);
