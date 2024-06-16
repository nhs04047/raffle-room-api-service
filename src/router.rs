use actix_web::web;

use crate::controllers::user_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(user_controller::create_user))
    );
}