
use std::{io::Result, time::Duration};
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use warp::{Reply, reject::Rejection, filters::ws::{Message, WebSocket}};

#[derive(Deserialize, Debug)]
struct WsRequest {
    kind: String,
    message: String,
}

#[derive(Serialize, Debug)]
struct WsResult {
    status: String,
    response: String,
}

pub async fn handle_ws_client(websocket: warp::ws::WebSocket) {
    // receiver - this server, from websocket client
    // sender - diff clients connected to this server
    let (mut sender, mut receiver) = websocket.split();

    while let Some(body) = receiver.next().await {
        let message = match body {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error reading message on websocket: {}", e);
                break;
            }
        };

        handle_websocket_message(message, &mut sender).await;
    }

    println!("client disconnected");
}

async fn handle_websocket_message(message: Message, sender: &mut SplitSink<WebSocket, Message>) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = message.to_str() {
        s
    } else {
        println!("ping-pong");
        return;
    };

    let req: WsRequest = serde_json::from_str(msg).unwrap();
    println!("got request {} with body {}", req.kind, req.message);

    std::thread::sleep(Duration::new(1, 0));

    let response = serde_json::to_string(&WsResult {
        status: "success".to_string(),
        response: "awesome message".to_string(),
    })
    .unwrap();
    sender.send(Message::text(response)).await.unwrap();
}

/*
.map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();

                // tx is message received direction

                // rx is message sent direction

                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        }); */