use wasm_bindgen::prelude::*;
use typst::{self};

use std::{self};

use futures_util::{FutureExt, StreamExt};
use warp::{self, Filter};

mod handler;

fn create_working_dir() -> Result<(), std::io::Error> {
    // Create if does not exist
    return std::fs::create_dir_all("./data");
}

#[tokio::main]
async fn main() {
    // Initialize working directory for the server where user data will be stored
    let dir_status = create_working_dir();
    match dir_status {
        Ok(_) => {
            println!("Working directory already exists or was successfully created!");
        }
        Err(error) => {
            panic!("Creation of server's working directory failed: {:?}", error);
        }
    };

    /*
    let load_settings = match  {

    }; {

    }
    */

    println!("Configuring WebSocket routes");

    let routes = warp::path("ws")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            println!("upgrading connection to websocket");
            ws.on_upgrade(handler::handle_ws_client)
        });

    println!("Starting WebSocket");

    warp::serve(routes).run(([0, 0, 0, 0], 3669)).await;
}

#[wasm_bindgen]
pub fn typst() {
    println!("Testing typst command")
}