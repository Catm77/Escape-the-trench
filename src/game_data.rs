use serde::{Serialize,Deserialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats
{
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player
{
    pub name: String,
    pub stats: Stats,
    pub stage: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enemy
{
    pub name: String,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameData 
{
    pub player: Player,
    pub enemies: Vec<Enemy>,
}