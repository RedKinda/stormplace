#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use stormplace::{PixelPaintRequest, PixelUpdate, PublicId};
use tauri::{async_runtime::Mutex, AppHandle, Manager, Window};
use tokio::time::{sleep, Duration};
use tonic::{transport::Channel, Request, Streaming};

use crate::stormplace::stormplace_client::StormplaceClient;

mod stormplace;

lazy_static::lazy_static! {
    static ref SPLACE_CLIENT: OnceCell<Mutex<StormplaceClient<Channel>>> = {
        OnceCell::new()
    };
}

async fn get_client() -> &'static Mutex<StormplaceClient<Channel>> {
    if SPLACE_CLIENT.get().is_none() {
        let client = StormplaceClient::connect("http://[::1]:50051")
            .await
            .unwrap();
        SPLACE_CLIENT.set(Mutex::new(client)).unwrap();
    }

    SPLACE_CLIENT.get().unwrap()
}

async fn process_update_stream(app: &mut AppHandle, mut stream: Streaming<PixelUpdate>) {
    println!("Got stream to process...");
    while let Ok(Some(update)) = stream.message().await {
        println!("Got update: {:?}", update);

        app.emit_all("pixel-update", (update.x, update.y, update.color))
            .unwrap();

        sleep(Duration::from_millis(10)).await;

        // Send tauri event
    }
    println!("Stream ended!");
}

async fn stream_changes(mut app: AppHandle) {
    let req = Request::new(PublicId {
        name: "redtest".to_string(),
    });

    let initial_req = Request::new(PublicId {
        name: "redtest".to_string(),
    });

    let mut client = get_client().await.lock().await;

    let initial_response = client.get_canvas_state_once(initial_req).await.unwrap();
    let initial_stream = initial_response.into_inner();

    let response = client.stream_changes(req).await.unwrap();
    let update_stream = response.into_inner();

    drop(client);

    process_update_stream(&mut app, initial_stream).await;

    process_update_stream(&mut app, update_stream).await;
}

#[tauri::command(async)]
async fn start_stream(window: Window) {
    tauri::async_runtime::spawn(stream_changes(window.app_handle()));
}

#[tauri::command(async)]
async fn paint_pixel(x: u64, y: u64, color: u32) {
    let client = get_client().await;
    let mut client = client.lock().await;

    client
        .paint_pixel(PixelPaintRequest {
            color,
            x,
            y,
            source: None,
        })
        .await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, paint_pixel, start_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// greet command
#[tauri::command]
fn greet() -> String {
    "Hello from Rust".to_string()
}
