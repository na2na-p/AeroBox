use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{key}")
            .route(web::post().to(crate::handlers::file::upload_file))
            .route(web::get().to(crate::handlers::file::get_file))
            .route(web::delete().to(crate::handlers::file::delete_file)),
    );
}
