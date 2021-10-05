use rand::seq::SliceRandom;
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
        "color": "#F09383",
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
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Step 0: Don't let your Battlesnake move back in on its own neck
    let my_head = &me.head;

    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.

    possible_moves.insert("left", valid_move(&my_head.left(), &board, &me));
    possible_moves.insert("right", valid_move(&my_head.right(), &board, &me));
    possible_moves.insert("up", valid_move(&my_head.up(), &board, &me));
    possible_moves.insert("down", valid_move(&my_head.down(), &board, &me));

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}

fn spot_has_snake(spot: &Coord, snakes: &Vec<Battlesnake>) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        snake_parts.push(snake.head);
        snake_parts.append(&mut snake.body.clone());
    }
    if snake_parts.contains(&spot) {
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
            body: vec![Coord { x: 3, y: 5 },
                       Coord { x: 4, y: 5 },
                       Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 0 },
            ],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 7 };
        assert_eq!(spot_has_snake(&spot, &snakes), false);
    }

    #[test]
    fn head_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![
                Coord { x: 3, y: 3 },
                Coord { x: 3, y: 2 },
            ],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 2, y: 3 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn tail_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![
                Coord { x: 3, y: 3 },
                Coord { x: 3, y: 2 },
            ],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 3, y: 2 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn hettie_is_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![Coord { x: 3, y: 5 },
                        Coord { x: 4, y: 5 },
                        Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 0 },
            ],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 0, y: 0 };
        assert_eq!(spot_has_snake(&spot, &snakes),  true);
    }

    #[test]
    fn i_am_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![Coord { x: 3, y: 5 },
                        Coord { x: 4, y: 5 },
                        Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 0 },
            ],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 5 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }
}


fn valid_move(spot: &Coord, board: &Board, me: &Battlesnake) -> bool {
    let board_width = board.width;
    let board_height = board.height;

    match spot {
        Coord { y: 0, .. } => false,
        Coord { x: 0, .. } => false,
        Coord { y, .. } if y == &board_width => false, // Rust is weird
        Coord { x, .. } if x == &board_height => false,
        spot if spot_has_snake(spot, &board.snakes) => false,
        _ => true,
    }
}

#[cfg(test)]
mod valid_move_tests {
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
        let spot = Coord { x: 0, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let spot = Coord { x: 5, y: 0 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
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
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, true);
    }
}
