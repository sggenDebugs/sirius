use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    routing::get,
    response::IntoResponse,
    Router,
};

use tokio::sync::{broadcast};
use uuid::Uuid;
use futures_util::stream::{StreamExt};
use futures_util::SinkExt;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

async fn ws_handler(ws: WebSocketUpgrade, state: axum::extract::State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: axum::extract::State<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
    let id = Uuid::new_v4();

    println!("Client {} connected", id);

    // Task to receive messages from this client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Task to send messages from this client to the broadcast channel
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let axum::extract::ws::Message::Text(text) = msg {
                // In a real app, you'd parse 'text' as JSON and validate it here
                let _ = state.tx.send(format!("User {}: {}", id, text));
            }
        }
    });

    // Wait for either task to finish
   tokio::select! {
       result = send_task => {
           if let Err(e) = result {
               eprintln!("Send task panicked: {}", e);
           }
       },
       result = recv_task => {
           if let Err(e) = result {
               eprintln!("Recv task panicked: {}", e);
           }
       },
   }

    println!("Client {} disconnected", id);
}

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = AppState { tx };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    println!("Listening on 0.0.0.0:80");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}