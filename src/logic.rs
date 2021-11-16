use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Coord, Game};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "ChaelCodes",
        "color": "#c88b4c",
        "head": "bendr",
        "tail": "round-bum",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, me: &Battlesnake) -> &'static str {
    let my_head = &me.head;

    let possible_moves: HashMap<_, _> = vec![
        ("up", value_of_move(&my_head.up(), board, me)),
        ("down", value_of_move(&my_head.down(), board, me)),
        ("left", value_of_move(&my_head.left(), board, me)),
        ("right", value_of_move(&my_head.right(), board, me)),
    ]
    .into_iter()
    .collect();

    let chosen = possible_moves.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;

    info!("{} MOVE {}", game.id, chosen);

    chosen
}

#[cfg(test)]
mod get_move_tests {
    use super::*;

    #[test]
    fn test_get_move() {
        let head = Coord { x: 9, y: 9 };
        let me = Battlesnake {
            body: vec![head.left()],
            head,
            ..Default::default()
        };
        let board = Board {
            height: 10,
            width: 10,
            snakes: vec![me.clone()],
            ..Default::default()
        };
        let game = Game::default();
        let get_move = get_move(&game, &0, &board, &me);
        assert_eq!(get_move, "down");
    }
}

fn spot_has_food(spot: &Coord, board: &Board) -> bool {
    board.food.contains(&spot)
}

#[cfg(test)]
mod spot_has_food_test {
    use super::*;

    #[test]
    fn hazardous_spot_test() {
        let board = Board {
            food: vec![
                Coord { x: 10, y: 0 },
                Coord { x: 7, y: 6 },
                Coord { x: 5, y: 3 },
            ],
            ..Default::default()
        };
        let spot = Coord { x: 7, y: 6 };
        assert_eq!(spot_has_food(&spot, &board), true);
    }

    #[test]
    fn safe_spot_test() {
        let board = Board {
            food: vec![
                Coord { x: 10, y: 0 },
                Coord { x: 7, y: 6 },
                Coord { x: 5, y: 3 },
            ],
            ..Default::default()
        };
        let spot = Coord { x: 3, y: 5 };
        assert_eq!(spot_has_food(&spot, &board), false);
    }
}

fn spot_has_hazards(spot: &Coord, board: &Board) -> bool {
    board.hazards.contains(spot)
}

#[cfg(test)]
mod spot_has_hazards_test {
    use super::*;

    #[test]
    fn hazardous_spot_test() {
        let board = Board {
            hazards: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 0, y: 2 },
                Coord { x: 0, y: 3 },
                Coord { x: 0, y: 4 },
                Coord { x: 0, y: 5 },
                Coord { x: 0, y: 6 },
                Coord { x: 0, y: 7 },
                Coord { x: 0, y: 8 },
                Coord { x: 0, y: 9 },
            ],
            ..Default::default()
        };
        let spot = Coord { x: 0, y: 5 };
        assert!(spot_has_hazards(&spot, &board));
    }

    #[test]
    fn safe_spot_test() {
        let board = Board {
            hazards: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 0, y: 1 },
                Coord { x: 0, y: 2 },
                Coord { x: 0, y: 3 },
                Coord { x: 0, y: 4 },
                Coord { x: 0, y: 5 },
                Coord { x: 0, y: 6 },
                Coord { x: 0, y: 7 },
                Coord { x: 0, y: 8 },
                Coord { x: 0, y: 9 },
            ],
            ..Default::default()
        };
        let spot = Coord { x: 3, y: 5 };
        assert!(!spot_has_hazards(&spot, &board));
    }
}

fn spot_has_snake(spot: &Coord, snakes: &[Battlesnake]) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        snake_parts.push(snake.head);
        snake_parts.append(&mut snake.body.clone());
    }
    if snake_parts.contains(spot) {
        return true;
    }

    false
}

#[cfg(test)]
mod spot_has_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 7 };
        assert!(!spot_has_snake(&spot, &snakes));
    }

    #[test]
    fn head_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 2, y: 3 };
        assert!(spot_has_snake(&spot, &snakes));
    }

    #[test]
    fn tail_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 3, y: 2 };
        assert!(spot_has_snake(&spot, &snakes));
    }

    #[test]
    fn hettie_is_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 0, y: 0 };
        assert!(spot_has_snake(&spot, &snakes));
    }

    #[test]
    fn i_am_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 5 };
        assert!(spot_has_snake(&spot, &snakes));
    }
}

fn spot_might_have_snake(spot: &Coord, snakes: &[Battlesnake], me: &Battlesnake) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        if snake.id != me.id && snake.length >= me.length {
            let head = snake.head;

            snake_parts.push(head.left());
            snake_parts.push(head.right());
            snake_parts.push(head.up());
            snake_parts.push(head.down());
        }
    }
    if snake_parts.contains(spot) {
        return true;
    }

    false
}

#[cfg(test)]
mod spot_might_have_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "Hettie".to_string(),
            head: Coord { x: 0, y: 0 },
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie, me.clone()];
        let spot = Coord { x: 5, y: 7 };
        assert!(!spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn larger_snake_head_right_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert!(spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn larger_snake_head_left_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.left();
        assert!(spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn same_size_snake_head_above_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.down();
        assert!(spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn same_size_snake_head_below_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.up();
        assert!(spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn smaller_snake_head_next_to_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 4,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert!(!spot_might_have_snake(&spot, &snakes, &me));
    }

    #[test]
    fn i_am_next_to_spot() {
        let head = Coord { x: 3, y: 5 };
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head,
            ..Default::default()
        };
        let snakes = vec![me.clone()];
        let spot = head.right();
        assert!(!spot_might_have_snake(&spot, &snakes, &me));
    }
}

fn remaining_space(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let available_spaces = check_spot_for_space(spot, board, me.length, [].to_vec());
    available_spaces.len() as i32
}

fn check_spot_for_space(spot: &Coord, board: &Board, my_length: i32, mut available_spaces: Vec<Coord>) -> Vec<Coord> {
    if my_length <= available_spaces.len() as i32 {
            return available_spaces;
    }
    if available_spaces.contains(spot) {
        return available_spaces;
    }

    if valid_move(spot, board) {
        available_spaces.push(*spot);

        if valid_move(&spot.right(), board) {
            available_spaces = check_spot_for_space(&spot.right(), board, my_length, available_spaces);
        }
        if valid_move(&spot.left(), board) {
            available_spaces = check_spot_for_space(&spot.left(), board, my_length, available_spaces);
        }
        if valid_move(&spot.up(), board) {
            available_spaces = check_spot_for_space(&spot.up(), board, my_length, available_spaces);
        }
        if valid_move(&spot.down(), board) {
            available_spaces = check_spot_for_space(&spot.down(), board, my_length, available_spaces);
        }
    }
    available_spaces
}

#[cfg(test)]
mod remaining_space {
    use super::*;

    #[test]
    fn spot_is_big_enough_for_corney() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            length: 4,
            ..Default::default()
        };
        let board = Board {
            height: 10,
            width: 10,
            snakes: vec![me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 0, y: 5 };
        assert_eq!(remaining_space(&spot, &board, &me), 4);
    }

    #[test]
    fn spot_is_too_small_for_corney() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 0, y: 8 },
                Coord { x: 1, y: 8 },
                Coord { x: 2, y: 8 },
                Coord { x: 3, y: 8 },
                Coord { x: 4, y: 8 },
                Coord { x: 5, y: 8 },
                Coord { x: 5, y: 9 },
                Coord { x: 4, y: 9 },
                Coord { x: 3, y: 9 }
            ],
            length: 7,
            ..Default::default()
        };
        let board = Board {
            height: 10,
            width: 10,
            snakes: vec![me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 2, y: 9 };
        assert_eq!(remaining_space(&spot, &board, &me), 3);
    }

    #[test]
    fn spot_to_the_left_is_corney_sized() {
        let head = Coord { x: 3, y: 8 };
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            head: head.clone(),
            body: vec![
                Coord { x: 3, y: 0 },
                Coord { x: 4, y: 0 },
                Coord { x: 5, y: 0 },
                Coord { x: 4, y: 1 },
                Coord { x: 5, y: 1 },
                Coord { x: 0, y: 2 },
                Coord { x: 1, y: 2 },
                Coord { x: 2, y: 2 },
            ],
            length: 7,
            ..Default::default()
        };
        let board = Board {
            height: 10,
            width: 10,
            snakes: vec![me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 2, y: 1 };
        assert_eq!(remaining_space(&spot, &board, &me), 7);
    }
}

fn spot_modifier(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let mut modifier = 0;
    if spot_might_have_snake(spot, &board.snakes, &me) {
        modifier -= 80;
    }
    if spot_has_food(spot, &board) {
        modifier += 75;
    } else if spot_has_hazards(spot, &board) {
        let leftover_health = me.health - 14;
        modifier -= 100 - leftover_health;
    }
    if remaining_space(spot, &board, &me) >= me.length {
        modifier += 50
    } else {
        modifier -= 80
    }
    modifier
}

#[cfg(test)]
mod spot_modifier_tests {
    use super::*;

    #[test]
    fn spot_with_hazards() {
        let board = Board {
            hazards: vec![
                Coord { x: 2, y: 0 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 4 },
                Coord { x: 2, y: 8 },
                Coord { x: 2, y: 10 },
            ],
            ..Default::default()
        };
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            health: 75,
            ..Default::default()
        };
        let spot = Coord { x: 2, y: 4 };
        assert_eq!(spot_modifier(&spot, &board, &me), 11);
    }

    #[test]
    fn spot_where_snakes_may_soon_be() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: Coord { x: 3, y: 5 },
            length: 4,
            ..Default::default()
        };
        let board = Board {
            snakes: vec![me.clone(), hettie],
            ..Default::default()
        };
        let spot = Coord { x: 3, y: 6 };
        assert_eq!(spot_modifier(&spot, &board, &me), -30);
    }

    #[test]
    fn spot_with_food() {
        let board = Board {
            food: vec![
                Coord { x: 3, y: 0 },
                Coord { x: 2, y: 6 },
                Coord { x: 10, y: 5 },
            ],
            ..Default::default()
        };
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            health: 75,
            ..Default::default()
        };
        let spot = Coord { x: 2, y: 6 };
        assert_eq!(spot_modifier(&spot, &board, &me), 125);
    }

    #[test]
    fn spot_with_food_and_hazard() {
        let board = Board {
            food: vec![
                Coord { x: 2, y: 4 },
                Coord { x: 2, y: 6 },
                Coord { x: 10, y: 5 },
            ],
            hazards: vec![
                Coord { x: 2, y: 0 },
                Coord { x: 2, y: 2 },
                Coord { x: 2, y: 4 },
                Coord { x: 2, y: 8 },
                Coord { x: 2, y: 10 },
            ],
            ..Default::default()
        };
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            health: 75,
            ..Default::default()
        };
        let spot = Coord { x: 2, y: 4 };
        assert_eq!(spot_modifier(&spot, &board, &me), 125);
    }
}

fn valid_move(spot: &Coord, board: &Board) -> bool {
    match spot {
        Coord { y: -1, .. } => false,
        Coord { x: -1, .. } => false,
        Coord { y, .. } if y == &board.width => false,
        Coord { x, .. } if x == &board.height => false,
        spot if spot_has_snake(spot, &board.snakes) => false,
        _ => true,
    }
}

// Returns the potential value of the move Cornelius
fn value_of_move(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let base_value = match spot {
        spot if spot_has_snake(spot, &board.snakes) => -99, // Bite someone else before you bite the dust!
        spot if !valid_move(&spot, &board) => -100,
        Coord { y: 0, .. } => 60,
        Coord { x: 0, .. } => 60,
        _ => 100,
    };

    base_value + spot_modifier(spot, &board, &me)
}

#[cfg(test)]
mod value_of_move_tests {
    use super::*;

    // Wall Tests
    #[test]
    fn head_will_not_hit_left_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: -1, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -180);
    }

    #[test]
    fn head_will_not_hit_right_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 10, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -180);
    }

    #[test]
    fn head_will_not_hit_roof() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 10 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -180);
    }

    #[test]
    fn head_will_not_hit_floor() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: -1 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -180);
    }

    // Collision Tests

    #[test]
    fn do_not_hit_me() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 4 }, Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -179);
    }

    #[test]
    fn do_not_bite_hettie() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 3, y: 2 }, Coord { x: 4, y: 2 }],
            ..Default::default()
        };
        let board = Board {
            snakes: vec![hettie, me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 4, y: 2 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, -179);
    }

    #[test]
    fn potential_snake_head() {
        let me = Battlesnake::default();
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head,
            length: 4,
            ..Default::default()
        };
        let spot = head.right();
        let board = Board {
            snakes: vec![me.clone(), hettie],
            ..Default::default()
        };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 70);
    }

    // Board Hazards/Dangers
    #[test]
    fn hazards_identified() {
        let me = Battlesnake {
            health: 65 + 14,
            ..Default::default()
        };
        let board = Board {
            hazards: vec![
                Coord { x: 10, y: 0 },
                Coord { x: 10, y: 1 },
                Coord { x: 10, y: 2 },
                Coord { x: 10, y: 3 },
                Coord { x: 10, y: 4 },
                Coord { x: 10, y: 5 },
                Coord { x: 10, y: 6 },
                Coord { x: 10, y: 7 },
                Coord { x: 10, y: 8 },
                Coord { x: 10, y: 9 },
            ],
            ..Default::default()
        };
        let spot = Coord { x: 10, y: 7 };
        let value_of_move = value_of_move(&spot, &board, &me);
        assert_eq!(value_of_move, 115);
    }

    #[test]
    fn superstitious_of_zero() {
        let me = Battlesnake::default();
        let board = Board {
            height: 10,
            width: 10,
            ..Default::default()
        };
        let spot = Coord { x: 0, y: 5 };
        let value_of_move = value_of_move(&spot, &board, &me);
        assert_eq!(value_of_move, 110);
    }

    #[test]
    fn eats_food() {
        let me = Battlesnake {
            health: 100,
            ..Default::default()
        };
        let board = Board {
            food: vec![Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let spot = Coord { x: 5, y: 5 };
        let value_of_move = value_of_move(&spot, &board, &me);
        assert_eq!(value_of_move, 225);
    }

    #[test]
    fn head_will_travel() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 9 }, Coord { x: 5, y: 8 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 150);
    }
}
