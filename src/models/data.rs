use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug)]
pub struct Data{
    #[serde(skip_deserializing)]
    pub id:i32,
    pub device:String,
    pub value :f32,
    #[serde(skip_deserializing)]
    pub date: String,
}