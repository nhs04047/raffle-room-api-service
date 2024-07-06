use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct BaseResponse<T> {
  pub result_code: u16,
  pub result_msg: String,
  pub result_data: Option<T>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorData {
  pub error_code: u16,
  pub error_msg: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse<T> {
  pub result_code: u16,
  pub result_msg: String,
  pub error_data: ErrorData,
  pub result_data: Option<T>

}