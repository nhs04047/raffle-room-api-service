use actix_web::web;

mod room;
mod draw_items;
mod joined_user;
mod draw;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        room::get_rooms
    );
}