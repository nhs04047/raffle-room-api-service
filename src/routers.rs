use actix_web::web;

mod room;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        room::get_rooms
    );
}