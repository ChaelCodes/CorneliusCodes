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
    let left = |head: &Coord| Coord {
        x: head.x - 1,
        y: head.y,
    };
    let right = |head: &Coord| Coord {
        x: head.x + 1,
        y: head.y,
    };
    let up = |head: &Coord| Coord {
        x: head.x,
        y: head.y + 1,
    };
    let down = |head: &Coord| Coord {
        x: head.x,
        y: head.y - 1,
    };
    possible_moves.insert("left", valid_move(&left(&my_head), &board, &me));
    possible_moves.insert("right", valid_move(&right(&my_head), &board, &me));
    possible_moves.insert("up", valid_move(&up(&my_head), &board, &me));
    possible_moves.insert("down", valid_move(&down(&my_head), &board, &me));

    // TODO: Step 2 - Don't hit yourself.
    // Use body information to prevent your Battlesnake from colliding with itself.
    // body = move_req.body

    // TODO: Step 3 - Don't collide with others.
    // Use snake vector to prevent your Battlesnake from colliding with others.
    // snakes = move_req.board.snakes

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

fn valid_move(spot: &Coord, board: &Board, me: &Battlesnake) -> bool {
    let board_width = board.width;
    let board_height = board.height;
    let my_neck = || { &me.body[1] };

    match spot {
        Coord { y: 0, .. } => { println!("down"); return false; },
        Coord { x: 0, .. } => { println!("left"); return false; },
        Coord { y, .. } if y == &board_width => { println!("right"); return false; }, // Rust is weird
        Coord { x, .. } if x == &board_height => { println!("up"); return false; },
        Coord { x, y } if x == &my_neck().x && y == &my_neck().y => { println!("my neck"); return false; },
        _ => { true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            snakes: vec![],
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
            snakes: vec![],
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
            snakes: vec![],
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
            snakes: vec![],
        };
        let spot = Coord { x: 5, y: 0 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, false);
    }

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
            snakes: vec![],
        };
        let spot = Coord { x: 5, y: 5 };
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
            snakes: vec![],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = valid_move(&spot, &board, &me);
        assert_eq!(valid_move, true);
    }
}
