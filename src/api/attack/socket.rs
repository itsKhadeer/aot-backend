use serde::{Deserialize, Serialize};

// use crate::validator::util::Coords;
use crate::validator::{
    self,
    util::{
        Attacker, BombType, BuildingDetails, BulletSpawnResponse, CompanionResult, Coords,
        DefenderDetails, MineDetails,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketRequest {
    pub frame_number: i32,
    pub action_type: ActionType,
    pub attacker_id: Option<i32>,
    pub bomb_id: Option<i32>,
    pub current_position: Option<Coords>,
    // pub attacker_path: Vec<Coords>,
    pub bomb_position: Coords,
    pub is_game_over: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketResponse {
    pub frame_number: i32,
    pub result_type: ResultType,
    pub is_alive: Option<bool>,
    pub attacker_health: Option<i32>,
    pub exploded_mines: Option<Vec<validator::util::MineResponse>>,
    // pub triggered_defenders: Option<Vec<DefenderResponse>>,
    pub defender_damaged: Option<Vec<DefenderResponse>>,
    pub hut_triggered: bool,
    pub hut_defenders: Option<Vec<DefenderDetails>>,
    pub damaged_base_items: Option<BaseItemsDamageResponse>,
    pub total_damage_percentage: Option<f32>,
    pub is_sync: bool,
    // pub state: Option<GameStateResponse>,
    pub is_game_over: bool,
    pub message: Option<String>,
    pub shoot_bullets: Option<Vec<BulletSpawnResponse>>,
    pub companion: Option<CompanionResult>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    IsMine,
    PlaceAttacker,
    PlaceCompanion,
    MoveAttacker,
    PlaceBombs,
    Idle,
    Terminate,
    SelfDestruct,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ResultType {
    MinesExploded,
    DefendersDamaged,
    DefendersTriggered,
    SpawnHutDefender,
    BuildingsDamaged,
    GameOver,
    PlacedAttacker,
    PlacedCompanion,
    Nothing,
}

#[derive(Serialize, Deserialize)]
pub struct MineResponse {
    pub id: i32,
    pub position: Coords,
    pub damage: i32,
    pub radius: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct DefenderResponse {
    pub map_space_id: i32,
    pub position: Coords,
    pub damage: i32,
    pub target_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BuildingDamageResponse {
    pub id: i32,
    pub position: Coords,
    pub hp: i32,
    pub artifacts_if_damaged: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DefenderDamageResponse {
    pub map_space_id: i32,
    pub position: Coords,
    pub health: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseItemsDamageResponse {
    pub buildings_damaged: Vec<BuildingDamageResponse>,
    pub defenders_damaged: Vec<DefenderDamageResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactsResponse {
    pub building_id: i32,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GameStateResponse {
    pub frame_no: i32,
    pub attacker_user_id: i32,
    pub defender_user_id: i32,
    pub attacker: Option<Attacker>,
    pub attacker_death_count: i32,
    pub bombs: BombType,
    pub damage_percentage: f32,
    pub artifacts: i32,
    pub defenders: Vec<DefenderDetails>,
    pub mines: Vec<MineDetails>,
    pub buildings: Vec<BuildingDetails>,
    pub total_hp_buildings: i32,
}
