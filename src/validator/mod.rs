use std::collections::{HashMap, HashSet};

use crate::{
    api::attack::{
        socket::{ActionType, BaseItemsDamageResponse, ResultType, SocketRequest, SocketResponse},
        util::{EventResponse, GameLog},
    },
    constants::COMPANION_BOT_RANGE,
    models::AttackerType,
    validator::util::{Coords, SourceDestXY},
};
use anyhow::{Ok, Result};
use util::{Companion, CompanionResult, MineResponse, Path};

use self::{
    state::State,
    util::{send_terminate_game_message, Attacker, BombType, DefenderReturnType},
};

pub mod error;
pub mod state;
pub mod util;

pub fn game_handler(
    attacker_type: &HashMap<i32, AttackerType>,
    socket_request: SocketRequest,
    _game_state: &mut State,
    _shortest_path: &HashMap<SourceDestXY, Path>,
    _roads: &HashSet<(i32, i32)>,
    _bomb_types: &Vec<BombType>,
    mut _game_log: &mut GameLog,
) -> Option<Result<SocketResponse>> {
    let defender_damaged_result: DefenderReturnType;
    let exploded_mines_result: Vec<MineResponse>;
    let base_items_damaged_result: BaseItemsDamageResponse;
    match socket_request.action_type {
        ActionType::PlaceAttacker => {
            _game_state.update_frame_number(socket_request.frame_number);
            let mut event_response = EventResponse {
                attacker_id: None,
                bomb_id: None,
                coords: Coords { x: 0, y: 0 },
                // direction: Direction::Up,
                is_bomb: false,
            };

            if let Some(attacker_id) = socket_request.attacker_id {
                let attacker: AttackerType = attacker_type.get(&attacker_id).unwrap().clone();
                _game_state.place_attacker(Attacker {
                    id: attacker.id,
                    // path_in_current_frame: Vec::new(),
                    attacker_pos: socket_request.current_position.unwrap(),
                    attacker_health: attacker.max_health,
                    attacker_speed: attacker.speed,
                    bombs: Vec::new(),
                    trigger_defender: false,
                    bomb_count: attacker.amt_of_emps,
                });

                for bomb_type in _bomb_types {
                    if let Some(bomb_id) = socket_request.bomb_id {
                        if bomb_type.id == bomb_id {
                            _game_state.set_bombs(bomb_type.clone(), attacker.amt_of_emps);
                        }
                    }
                }

                event_response.attacker_id = Some(attacker_id);
                event_response.coords = socket_request.current_position.unwrap();
            }

            // _game_state.set_mines(mine_positions);
            event_response.bomb_id = socket_request.bomb_id;

            _game_log.e.push(event_response);
            _game_log.r.au += 1;

            if _game_state.in_validation.is_invalidated {
                return Some(Ok(send_terminate_game_message(
                    socket_request.frame_number,
                    _game_state.in_validation.message.clone(),
                )));
            }

            for defender in _game_state.defenders.iter() {
                log::info!(
                    "defender id : {} , position x {}, y {} ",
                    defender.map_space_id,
                    defender.defender_pos.x,
                    defender.defender_pos.y
                );
            }

            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);

            return Some(Ok(SocketResponse {
                frame_number: socket_request.frame_number,
                result_type: ResultType::PlacedAttacker,
                is_alive: Some(true),
                attacker_health,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: None,
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                shoot_bullets: None,
                message: Some(String::from(
                    "Place Attacker, set attacker and bomb response",
                )),
                companion: None,
            }));
        }
        ActionType::PlaceCompanion => {
            _game_state.update_frame_number(socket_request.frame_number);

            if let Some(attacker_id) = socket_request.attacker_id {
                let attacker: AttackerType = attacker_type.get(&attacker_id).unwrap().clone();
                _game_state.place_companion(Companion {
                    id: attacker.id,
                    path_in_current_frame: Vec::new(),
                    companion_pos: socket_request.current_position.unwrap(),
                    companion_health: attacker.max_health,
                    companion_speed: attacker.speed,
                    bombs: Vec::new(),
                    trigger_defender: false,
                    bomb_count: attacker.amt_of_emps,
                    range: COMPANION_BOT_RANGE,
                    target_building: None,
                    target_defender: None,
                    target_tile: None,
                    current_target: None,
                    reached_dest: false,
                    last_attack_tick: 0,
                    attack_interval: 10,
                    damage: 30,
                });

                for bomb_type in _bomb_types {
                    if let Some(bomb_id) = socket_request.bomb_id {
                        if bomb_type.id == bomb_id {
                            _game_state
                                .set_companion_bombs(bomb_type.clone(), attacker.amt_of_emps);
                        }
                    }
                }
            }
            return Some(Ok(SocketResponse {
                frame_number: socket_request.frame_number,
                result_type: ResultType::PlacedCompanion,
                is_alive: Some(true),

                attacker_health: None,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: None,
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                message: Some(String::from("Placed companion")),
                companion: None,
                shoot_bullets: None,
            }));
        }

        ActionType::MoveAttacker => {
            if let Some(attacker_id) = socket_request.attacker_id {
                let attacker: AttackerType = attacker_type.get(&attacker_id).unwrap().clone();
                // let attacker_delta: Vec<Coords> = socket_request.attacker_path;
                // let attacker_delta_clone = attacker_delta.clone();

                let _attacker_result = _game_state.attacker_movement(
                    socket_request.frame_number,
                    _roads,
                    Attacker {
                        id: attacker.id,
                        // path_in_current_frame: attacker_delta.clone(),
                        attacker_pos: socket_request.current_position.unwrap(),
                        attacker_health: attacker.max_health,
                        attacker_speed: attacker.speed,
                        bombs: Vec::new(),
                        trigger_defender: false,
                        bomb_count: attacker.amt_of_emps,
                    },
                );

                // let attacker_result_clone = attacker_result.clone().unwrap();

                defender_damaged_result = _game_state
                    .defender_movement_one_tick(socket_request.current_position?, _shortest_path);

                let mut is_attacker_alive = true;

                if let Some(attacker) = &_game_state.attacker {
                    if attacker.attacker_health == 0 {
                        is_attacker_alive = false;
                    }
                }

                if _game_state.in_validation.is_invalidated {
                    return Some(Ok(send_terminate_game_message(
                        socket_request.frame_number,
                        _game_state.in_validation.message.clone(),
                    )));
                }

                let spawn_result = _game_state
                    .spawn_hut_defender(
                        _roads,
                        // Attacker {
                        //     id: attacker.id,
                        //     path_in_current_frame: attacker_delta_clone.clone(),
                        //     attacker_pos: socket_request.start_position.unwrap(),
                        //     attacker_health: attacker.max_health,
                        //     attacker_speed: attacker.speed,
                        //     bombs: Vec::new(),
                        //     trigger_defender: false,
                        //     bomb_count: attacker.amt_of_emps,
                        // },
                    )
                    .unwrap();

                _game_state.activate_sentry();
                let shoot_bullets = _game_state.shoot_bullets();
                if _game_state.attacker.is_some() || _game_state.attacker.is_some() {
                    _game_state.cause_bullet_damage();
                }

                let companion_res = _game_state
                    .move_companion(_roads, _shortest_path)
                    .unwrap_or(CompanionResult {
                        current_target: None,
                        map_space_id: -1,
                        current_target_tile: None,
                        is_alive: false,
                        health: -1,
                        building_damaged: None,
                        defender_damaged: None,
                    });

                _game_state.defender_trigger();

                let hut_triggered = !spawn_result.is_empty();

                let result_type = if hut_triggered {
                    ResultType::SpawnHutDefender
                } else if !defender_damaged_result.clone().defender_response.is_empty() {
                    ResultType::DefendersDamaged
                } else {
                    ResultType::BuildingsDamaged
                };

                let buildings_damaged =
                    if let Some(building_damaged) = &companion_res.building_damaged {
                        vec![building_damaged.clone()]
                    } else {
                        Vec::new()
                    };

                let defenders_damaged =
                    if let Some(defender_damaged) = &companion_res.defender_damaged {
                        vec![defender_damaged.clone()]
                    } else {
                        Vec::new()
                    };

                let damaged_base_items = Some(BaseItemsDamageResponse {
                    buildings_damaged,
                    defenders_damaged,
                });

                let response = SocketResponse {
                    frame_number: socket_request.frame_number,
                    result_type,
                    is_alive: Some(is_attacker_alive),
                    attacker_health: Some(defender_damaged_result.clone().attacker_health),
                    exploded_mines: None,
                    // triggered_defenders: Some(defender_damaged_result.clone().defender_response),
                    defender_damaged: Some(defender_damaged_result.clone().defender_response),
                    damaged_base_items,
                    hut_triggered,
                    hut_defenders: Some(spawn_result),
                    total_damage_percentage: Some(_game_state.damage_percentage),
                    is_sync: false,
                    is_game_over: false,
                    shoot_bullets: Some(shoot_bullets),
                    message: Some(String::from("Movement Response")),
                    companion: Some(companion_res),
                };
                return Some(Ok(response));
            }
        }
        ActionType::IsMine => {
            // is_mine
            let start_pos: Option<Coords> = socket_request.current_position;
            exploded_mines_result = _game_state.mine_blast(start_pos);

            let mut bool_temp = false;
            if !exploded_mines_result.is_empty() {
                bool_temp = true;
            }
            let result_type = if bool_temp {
                ResultType::MinesExploded
            } else {
                ResultType::Nothing
            };

            let mut is_attacker_alive = true;

            if let Some(attacker) = &_game_state.attacker {
                if attacker.attacker_health == 0 {
                    is_attacker_alive = false;
                }
            }

            if _game_state.in_validation.is_invalidated {
                return Some(Ok(send_terminate_game_message(
                    socket_request.frame_number,
                    _game_state.in_validation.message.clone(),
                )));
            }

            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);

            return Some(Ok(SocketResponse {
                frame_number: socket_request.frame_number,
                result_type,
                is_alive: Some(is_attacker_alive),
                attacker_health,
                exploded_mines: Some(exploded_mines_result),
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: None,
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                shoot_bullets: None,
                message: Some(String::from("Is Mine Response")),
                companion: None,
            }));
        }
        ActionType::PlaceBombs => {
            // let attacker_delta: Vec<Coords> = socket_request.attacker_path.clone();
            let current_pos = socket_request.current_position.unwrap();
            let bomb_coords = socket_request.bomb_position;

            if _game_state.bombs.total_count == 0 {
                return Some(Ok(send_terminate_game_message(
                    socket_request.frame_number,
                    "No bombs left".to_string(),
                )));
            }

            // for coord in attacker_delta.clone() {
            //     let mut direction = Direction::Up;

            //     let prev_pos = _game_log.e.last().unwrap().coords;
            //     if prev_pos.x < coord.x {
            //         direction = Direction::Down;
            //     } else if prev_pos.x > coord.x {
            //         direction = Direction::Up;
            //     } else if prev_pos.y < coord.y {
            //         direction = Direction::Left;
            //     } else if prev_pos.y > coord.y {
            //         direction = Direction::Right;
            //     }

            //     let event_response = EventResponse {
            //         attacker_id: None,
            //         bomb_id: None,
            //         coords: coord,
            //         direction,
            //         is_bomb: coord == bomb_coords,
            //     };

            //     _game_log.e.push(event_response.clone());
            // }

            base_items_damaged_result = _game_state.place_bombs(current_pos, bomb_coords);

            _game_log.r.b += 1;
            _game_log.r.d = _game_state.damage_percentage as i32;
            _game_log.r.a = _game_state.artifacts;

            let mut bool_temp = false;
            if !base_items_damaged_result.buildings_damaged.is_empty()
                || !base_items_damaged_result.defenders_damaged.is_empty()
            {
                bool_temp = true;
            }
            let result_type = if bool_temp {
                ResultType::BuildingsDamaged
            } else {
                ResultType::Nothing
            };

            if _game_state.in_validation.is_invalidated {
                return Some(Ok(send_terminate_game_message(
                    socket_request.frame_number,
                    _game_state.in_validation.message.clone(),
                )));
            }

            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);

            return Some(Ok(SocketResponse {
                frame_number: socket_request.frame_number,
                result_type,
                is_alive: Some(true),
                attacker_health,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: Some(base_items_damaged_result),
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                shoot_bullets: None,
                message: Some(String::from("Place Bomb Response")),
                companion: None,
            }));
        }
        ActionType::Idle => {
            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);
            return Some(Ok(SocketResponse {
                frame_number: socket_request.frame_number,
                result_type: ResultType::Nothing,
                is_alive: Some(true),
                attacker_health,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: None,
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                shoot_bullets: None,
                message: Some(String::from("Idle Response")),
                companion: None,
            }));
        }
        ActionType::Terminate => {
            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);
            let socket_response = SocketResponse {
                frame_number: socket_request.frame_number,
                result_type: ResultType::GameOver,
                is_alive: None,
                attacker_health,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: None,
                damaged_base_items: None,
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: true,
                shoot_bullets: None,
                message: Some(String::from("Game over")),
                companion: None,
            };
            return Some(Ok(socket_response));
        }
        ActionType::SelfDestruct => {
            _game_state.self_destruct();
            let attacker_health = _game_state
                .attacker
                .as_ref()
                .map(|attacker| attacker.attacker_health);
            let socket_response = SocketResponse {
                frame_number: socket_request.frame_number,
                result_type: ResultType::Nothing,
                is_alive: Some(false),
                attacker_health,
                exploded_mines: None,
                // triggered_defenders: None,
                defender_damaged: Some(Vec::new()),
                damaged_base_items: Some(BaseItemsDamageResponse {
                    buildings_damaged: Vec::new(),
                    defenders_damaged: Vec::new(),
                }),
                hut_triggered: false,
                hut_defenders: None,
                total_damage_percentage: Some(_game_state.damage_percentage),
                is_sync: false,
                is_game_over: false,
                shoot_bullets: None,
                message: Some(String::from("Self Destructed")),
                companion: None,
            };

            return Some(Ok(socket_response));
        }
    }
    None
}
