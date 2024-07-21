use crate::{
  repository::structs::{
    NewRoomModel, NewUserModel, UpdateRoomModel, NewDrawItemModel
  },
  router::dto, 
  service::structs::{
    Draw, NewDrawItem, NewRoom, NewUser, Room, UpdateRoom, User, DrawItem
  }
};
use super::structs::{
  SetDrawIncludeOwnerFlag, SetDrawOrderFlag, RoomStatusFlag
};
use chrono::Utc;
use entity::{
  room::Model as RoomModel,
  joined_user::Model as JoinedUserModel,
  draw::Model as DrawModle,
  draw_item::Model as DrawItemModel
};
use rand::Rng;

pub trait Mapper<From, To> {
  fn map(from: From) -> To;
}

pub trait MapperWithId<Id, From, To> {
  fn map_with_id(id: Id, from: From) -> To;
}


impl Mapper<dto::request::room::NewRoomDto, Self> for NewRoom {
  fn map(dto: dto::request::room::NewRoomDto) -> Self {
    let set_draw_include_owner = match dto.set_draw_include_owner {
      0 => SetDrawIncludeOwnerFlag::NotIncluded,
      1 => SetDrawIncludeOwnerFlag::Included,
      _ => SetDrawIncludeOwnerFlag::NotIncluded
    };
    
    let set_draw_order = match dto.set_draw_order.as_str() {
      "bulk" => SetDrawOrderFlag::Bulk,
      "sequential" => SetDrawOrderFlag::Sequential,
      _ => SetDrawOrderFlag::Bulk
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

impl Mapper<Self, NewRoomModel> for NewRoom {
  fn map(entity: Self) -> NewRoomModel {
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

impl Mapper<RoomModel, Self> for Room {
    fn map(model: RoomModel) -> Self {
      let set_draw_include_owner = match model.set_draw_include_owner {
        0 => SetDrawIncludeOwnerFlag::NotIncluded,
        1 => SetDrawIncludeOwnerFlag::Included,
        _ => SetDrawIncludeOwnerFlag::NotIncluded
      };
      
      let set_draw_order = match model.set_draw_order.as_str() {
        "bulk" => SetDrawOrderFlag::Bulk,
        "sequential" => SetDrawOrderFlag::Sequential,
        _ => SetDrawOrderFlag::Bulk
      };
  
      let status = match model.status {
        0 => RoomStatusFlag::RecruitingParticipants,
        1=> RoomStatusFlag::RecruitmentClosed,
        2=> RoomStatusFlag::DrawCompleted,
        _ => RoomStatusFlag::RecruitingParticipants
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

impl Mapper<Self, UpdateRoomModel> for UpdateRoom {
    fn map(entity: Self) -> UpdateRoomModel {

      let set_draw_include_owner = match entity.set_draw_include_owner {
        Some(SetDrawIncludeOwnerFlag::NotIncluded) => Some(0),
        Some(SetDrawIncludeOwnerFlag::Included) => Some(1),
        None => None,
    };

    let set_draw_order = match entity.set_draw_order {
        Some(SetDrawOrderFlag::Bulk) => Some("bulk".to_owned()),
        Some(SetDrawOrderFlag::Sequential) => Some("sequential".to_owned()),
        None => None,
    };

    let status = match entity.status {
      Some(RoomStatusFlag::RecruitingParticipants) => Some(0),
      Some(RoomStatusFlag::RecruitmentClosed) => Some(1),
      Some(RoomStatusFlag::DrawCompleted) => Some(2),
      None => None,
    };

      UpdateRoomModel {
        id: entity.id,
        name: entity.name,
        password: entity.password,
        set_draw_include_owner,
        set_draw_order,
        status,
        updated_at: entity.updated_at
      }
    }
}

impl MapperWithId<i32, dto::request::room::UpdateRoomDto, Self> for UpdateRoom {
  fn map_with_id(id: i32, entity: dto::request::room::UpdateRoomDto) -> Self {

    let set_draw_include_owner = match entity.set_draw_include_owner {
      Some(0) => Some(SetDrawIncludeOwnerFlag::NotIncluded),
      Some(1) => Some(SetDrawIncludeOwnerFlag::Included),
      Some(_) | None => None,
    };

    let set_draw_order = match entity.set_draw_order {
      Some(v) => {
        match v.as_str() {
          "bulk" => Some(SetDrawOrderFlag::Bulk),
          "sequential" => Some(SetDrawOrderFlag::Sequential),
          _ => None
        }
      },
      None => None
    };

    Self {
        id,
        name: entity.name,
        password: entity.password,
        set_draw_include_owner,
        set_draw_order,
        status: None,
        updated_at: Utc::now().naive_utc()
      }
  }
}

impl MapperWithId<i32, dto::request::joined_user::NewUserDto, Self> for NewUser {
  fn map_with_id(room_id: i32, dto: dto::request::joined_user::NewUserDto) -> Self {
    let random_number: u32 = rand::thread_rng().gen_range(1000..10000);
    let tag = format!("#{:04}", random_number);

    NewUser {
      name: dto.name,
      room_id,
      tag,
      created_at: dto.created_at
    }
  }
}

impl Mapper<Self, NewUserModel> for NewUser {
  fn map(entity: Self) -> NewUserModel {
    NewUserModel{
      name: entity.name,
      tag: entity.tag,
      room_id: entity.room_id,
      created_at: entity.created_at
      }
  }
}

impl Mapper<JoinedUserModel, Self> for User {
  fn map(model: JoinedUserModel) -> Self {
    Self {
        id: model.id,
        name: model.name,
        tag: model.tag,
        room_id: model.room_id,
        created_at: model.created_at,
    }
  }
}

impl Mapper<Self, dto::response::joined_user::UserDto> for User{
  fn map(entity: Self) -> dto::response::joined_user::UserDto {
    dto::response::joined_user::UserDto {
        id: entity.id,
        name: entity.name,
        tag: entity.tag,
        room_id: entity.room_id,
        created_at: entity.created_at,
    }
  }
}

impl Mapper<DrawModle, Self> for Draw {
  fn map(model: DrawModle) -> Self {
    Self {
      id: model.id,
      room_id: model.room_id,
      user_id: model.user_id,
      draw_item_id: model.draw_item_id,
      created_at: model.created_at,
    }
  }
}

impl MapperWithId<i32, dto::request::draw_item::NewDrawItemsDto, Self> for NewDrawItem {
  fn map_with_id(room_id: i32, dto:  dto::request::draw_item::NewDrawItemsDto) -> Self {
    NewDrawItem {
      room_id,
      name: dto.name,
      seq: dto.seq,
      qty: dto.qty,
      created_at: dto.created_at,
    }
  }
}

impl Mapper<Self, NewDrawItemModel> for NewDrawItem {
  fn map(entity: Self) -> NewDrawItemModel {
    NewDrawItemModel{
      name: entity.name,
      seq: entity.seq,
      room_id: entity.room_id,
      qty: entity.qty,
      created_at: entity.created_at
    }
  }
}

impl Mapper<DrawItemModel, Self> for DrawItem {
  fn map(model: DrawItemModel) -> Self {
    Self {
      id: model.id,
      room_id: model.room_id,
      name: model.name,
      seq: model.seq,
      qty: model.qty,
      created_at: model.created_at
    }
  }
}
