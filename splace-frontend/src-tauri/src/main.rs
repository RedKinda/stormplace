#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use stormplace::stormplace_client;

mod stormplace;

lazy_static::lazy_static! {
    static ref SPLACE_CLIENT: stormplace_client::StormplaceClient<tonic::transport::Channel> = {
        stormplace_client::StormplaceClient::new(tonic::transport::Channel::from_static("http://localhost:50051"))
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let client = stormplace_client::StormplaceClient::connect("http://localhost:50051")
                    .await
                    .unwrap();

                let req = tonic::Request::new(stormplace::PublicId {
                    name: "redtest".to_string(),
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// greet command
#[tauri::command]
fn greet() -> String {
    "Hello from Rust".to_string()
}
