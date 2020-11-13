use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Interactive {
    Item(Item),
    Container(Container),
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Room(String),
    NoGo(String),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub file: String,
    pub description: String,
    pub inventory: Vec<Interactive>,
    pub north: Direction,
    pub northeast: Direction,
    pub east: Direction,
    pub southeast: Direction,
    pub south: Direction,
    pub southwest: Direction,
    pub west: Direction,
    pub northwest: Direction,
    pub up: Direction,
    pub down: Direction,
    pub i: Direction,
    pub o: Direction,
}
/*
impl Room {
    fn look(&self) -> String {
        // recurse through inventory and return formatted readout
        String::from("")
    }
}
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub inventory: Vec<Item>,
    pub health: HealthStatus,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum HealthStatus {
    Healthy,
    Injured(String),
    Dead,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub size: u8,
    pub value: u8,
    pub take: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub name: String,
    pub desc: String,
    pub capacity: u8,
    pub open: bool,
    pub inventory: Vec<Interactive>,
}
