// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{error::Error, collections::HashMap};

mod models;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn request(method: &str, url: &str) -> String {
    let response = do_request(method, url);
    // TODO: Must unpack the HaspMap<String, String> into an string payload to be printed
    format!("{:#?}", response)
}

fn do_request(_method: &str, url: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let body = reqwest::blocking::get("https://httpbin.org/get?arg=blargh")
        .unwrap()
        .json::<HashMap<String, String>>()
        .unwrap();
    Ok(body)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
