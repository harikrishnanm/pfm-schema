use actix_web::web;

mod object_schema;

pub fn register(app: &mut web::ServiceConfig) {
    app.service(
        web::resource("/platfom/core/schema").route(web::post().to(object_schema::register_schema)),
    );
    /*.service(
        web::resource("/auth/validate/v1")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::post().to(token::validate))
    )
    .service(
        web::resource("/user/register/v1")
            .guard(guard::Header("content-type", "application/json"))
            .route(web::post().to(user::register))
    )
    .service(
        web::resource("/user/update/v1")
            .wrap(auth)
            .guard(guard::Header("content-type", "application/json"))
            .route(web::put().to(user::update))
    )
    .service(
        web::resource("/user/v1/{userid}")
            .route(web::get().to(user::get))
    );*/
}
