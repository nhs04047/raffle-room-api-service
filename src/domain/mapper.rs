use crate::{service::structs::{NewRoom, Room}, router::dto, repository::structs::NewRoomModel};
use super::structs::{SetDrawIncludeOwnerFlag, SetDrawOrderFlag, RoomStatusFlag};

use entity::
  room::Model as RoomModel;

impl NewRoom {
  pub fn from_dto(dto: dto::request::room::NewRoomDto) -> Self {
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

  pub fn to_model(entity: Self) -> NewRoomModel {
    let set_draw_include_owner = match entity.set_draw_include_owner {
      SetDrawIncludeOwnerFlag::NotIncluded => 0,
      SetDrawIncludeOwnerFlag::Included => 1
    };
    let set_draw_order = match entity.set_draw_order {
        SetDrawOrderFlag::Bulk => "bulk".to_owned(),
        SetDrawOrderFlag::Sequential => "sequential".to_owned(),
    };
    let status = match entity.status {
        RoomStatusFlag::RecruitingParticipants => 0,
        RoomStatusFlag::RecruitmentClosed => 1,
        RoomStatusFlag::DrawCompleted => 2,
    };

    NewRoomModel {
      name: entity.name,
      password: entity.password,
      set_draw_include_owner,
      set_draw_order,
      status,
      created_at: entity.created_at,
      updated_at: entity.updated_at
    }
  }

}

impl Room {
  pub fn from_model(model: RoomModel) -> Self {
    let set_draw_include_owner = match model.set_draw_include_owner {
      0 => SetDrawIncludeOwnerFlag::NotIncluded,
      1 => SetDrawIncludeOwnerFlag::Included
    };
    
    let set_draw_order = match model.set_draw_order.as_str() {
      "bulk" => SetDrawOrderFlag::Bulk,
      "sequential" => SetDrawOrderFlag::Sequential
    };

    let status = match model.status {
      0 => RoomStatusFlag::RecruitingParticipants,
      1=> RoomStatusFlag::RecruitmentClosed,
      2=> RoomStatusFlag::DrawCompleted,
    };

    Self {
      id: model.id,
      name: model.name,
      password: model.password,
      set_draw_include_owner,
      set_draw_order,
      status: RoomStatusFlag::RecruitingParticipants,
      created_at: model.created_at,
      updated_at: model.created_at
    }
  }
}