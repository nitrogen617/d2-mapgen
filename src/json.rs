
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct SeedData
{
    pub seed: u32,
    pub difficulty: u32,
    pub levels: Vec<LevelData>,
}

pub struct LevelDump {
    pub level: LevelData,
    pub rooms: Vec<RoomData>,
    pub adjacency: Vec<Adjacency>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct LevelData {
    pub id: u32,
    pub name: String,
    pub offset: Offset,
    pub size: Size,
    pub objects: Vec<Object>,
    pub map: Vec<Vec<u64>>,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct RoomData {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Default, Debug, Clone)]
pub struct Adjacency {
    pub id: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct KooloLevel {
    #[serde(rename = "type")]
    pub level_type: String,
    pub id: u32,
    pub name: String,
    pub offset: Offset,
    pub size: Size,
    pub objects: Vec<KooloObject>,
    pub rooms: Vec<RoomData>,
    pub map: Vec<Vec<u64>>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct KooloObject {
    pub id: u32,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub x: i32,
    pub y: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "isGoodExit")]
    pub is_good_exit: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub unit_type: String,
    pub x: u32,
    pub y: u32,
     #[serde(skip_serializing_if = "Option::is_none")]
    pub is_good_exit: Option<bool>
}
