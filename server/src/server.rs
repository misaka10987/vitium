use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    thread::sleep,
    time,
};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use vitium_common::{
    chara::Character,
    item::Item,
    player::{Player, Token},
    registry::RegTable,
    scene::Scene,
    skill::{Prof, Skill},
    vehicle::Vehicle,
};

use crate::UNTIL;

pub struct Server<'a> {
    pub name: &'a str,
    reg_item: RegTable<'a, Item>,
    reg_skill: RegTable<'a, Skill>,
    reg_prof: RegTable<'a, Prof>,
    reg_scene: RegTable<'a, Scene>,
    reg_vehicle: RegTable<'a, Vehicle>,
    player: RefCell<HashMap<String, Player>>,
    pswd: RefCell<HashMap<String, String>>,
    character: RefCell<HashMap<String, Character>>,
    // todo
    game: (),
}

impl<'a> Server<'a> {
    // pub fn verify(&self, token: &Token) -> bool {
    //     if let Some(pswd) = self.pswd.get(&token.id) {
    //         pswd == &token.pswd
    //     } else {
    //         false
    //     }
    // }
    fn player(&self) -> Ref<'_, HashMap<String, Player>> {
        loop {
            if let Ok(data) = self.player.try_borrow() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    fn player_mut(&self) -> RefMut<'_, HashMap<String, Player>> {
        loop {
            if let Ok(data) = self.player.try_borrow_mut() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    fn pswd(&self) -> Ref<'_, HashMap<String, String>> {
        loop {
            if let Ok(data) = self.pswd.try_borrow() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    fn pswd_mut(&self) -> RefMut<'_, HashMap<String, String>> {
        loop {
            if let Ok(data) = self.pswd.try_borrow_mut() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    fn character(&self) -> Ref<'_, HashMap<String, Character>> {
        loop {
            if let Ok(data) = self.character.try_borrow() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    fn character_mut(&self) -> RefMut<'_, HashMap<String, Character>> {
        loop {
            if let Ok(data) = self.character.try_borrow_mut() {
                break data;
            }
            sleep(time::Duration::from_millis(UNTIL));
        }
    }
    pub fn verify(&self, token: &Token) -> bool {
        if let Some(pswd) = self.pswd().get(&token.id) {
            pswd == &token.pswd
        } else {
            false
        }
    }
    #[tokio::main]
    pub async fn run(&self) {
        let app = Router::new()
            // `GET /` goes to `root`
            .route("/", get(root))
            // `POST /users` goes to `create_user`
            .route("/users", post(create_user));
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
