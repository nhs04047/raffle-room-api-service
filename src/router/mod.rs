use actix_web::web;

pub mod dto;

mod room;
mod draw_item;
mod joined_user;
mod draw;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(room::get_rooms);
    cfg.service(room::get_room);
    cfg.service(room::create_rooms);
    cfg.service(room::update_room);
    cfg.service(draw_item::get_draw_items);
    cfg.service(draw_item::create_draw_items);
    cfg.service(draw::get_draws);
    cfg.service(joined_user::create_user);
    cfg.service(joined_user::get_user);
    cfg.service(joined_user::get_users);
    
}