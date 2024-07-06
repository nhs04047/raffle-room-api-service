use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum SetDrawIncludeOwnerFlag {
  NotIncluded = 0,
  Included = 1,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SetDrawOrderFlag {
  Bulk,
  Sequential,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RoomStatusFlag {
  RecruitingParticipants = 0,
  RecruitmentClosed = 1,
  DrawCompleted = 2,
}