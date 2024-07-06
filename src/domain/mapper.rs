use crate::{service::structs::NewRoom, router::dto};
use super::structs::{SetDrawIncludeOwnerFlag, SetDrawOrderFlag, RoomStatusFlag};
use entity;

impl NewRoom {
  fn from_dto(dto: dto::request::room::NewRoomDto) -> Self {
    let set_draw_include_owner = match dto.set_draw_include_owner {
      0 => SetDrawIncludeOwnerFlag::NotIncluded,
      1 => SetDrawIncludeOwnerFlag::Included
    };
    
    let set_draw_order = match dto.set_draw_order.as_str() {
      "bulk" => SetDrawOrderFlag::Bulk,
      "sequential" => SetDrawOrderFlag::Sequential
    };

    NewRoom {
      name: dto.name,
      password: dto.password,
      set_draw_include_owner,
      set_draw_order,
      status: RoomStatusFlag::RecruitingParticipants,
      created_at: dto.created_at,
      updated_at: dto.created_at
    }
  }
}