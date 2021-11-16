#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use log::info;
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

mod logic;

// Request types derived from https://docs.battlesnake.com/references/api#object-definitions
// For a full example of Game Board data, see https://docs.battlesnake.com/references/api/sample-move-request

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    body: Vec<Coord>,
    head: Coord,
    health: i32,
    id: String,
    latency: String,
    length: i32,
    name: String,

    // Used in non-standard game modes
    shout: Option<String>,
    squad: Option<String>,
}

impl Default for Battlesnake {
    fn default() -> Battlesnake {
        Self {
            body: Vec::<Coord>::default(),
            head: Coord::default(),
            health: 100,
            id: "CorneliusCodes".to_string(),
            latency: String::default(),
            length: 4,
            name: "CorneliusCodes".to_string(),

            // Used in non-standard game modes
            shout: Option::<String>::default(),
            squad: Option::<String>::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Board {
    food: Vec<Coord>,
    hazards: Vec<Coord>,
    height: i32,
    snakes: Vec<Battlesnake>,
    width: i32,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Coord {
    x: i32,
    y: i32,
}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coord {}
impl Coord {
    pub fn down(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    board: Board,
    game: Game,
    turn: u32,
    you: Battlesnake,
}

#[get("/")]
fn handle_index() -> JsonValue {
    logic::get_info()
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> JsonValue {
    let chosen = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    return json!({ "move": chosen });
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

fn main() {
    let address = "0.0.0.0";
    let env_port = env::var("PORT").ok();
    let env_port = env_port.as_deref().unwrap_or("8080");
    let port = env_port.parse::<u16>().unwrap();

    env_logger::init();

    let config = Config::build(Environment::Development)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();

    info!(
        "Starting Battlesnake Server at http://{}:{}...",
        address, port
    );
    rocket::custom(config)
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
        .launch();
}
