#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[cfg(target_os = "macos")]
static STEAMCMD_EXENAME: &str = "steamcmd";
#[cfg(target_os = "macos")]
static STEAMCMD_DOWNLOADURL: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz";
#[cfg(target_os = "windows")]
static STEAMCMD_EXENAME: &str = "steamcmd.exe";
#[cfg(target_os = "windows")]
static STEAMCMD_DOWNLOADURL: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.tar.gz";
use flate2::read::GzDecoder;
use reqwest::blocking::get;
use std::env;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use subprocess::Exec;
use tar::Archive;

#[tauri::command]
async fn helpme(hello: String) -> String {
    format!("Arg {}", hello)
}

fn main() {
    let mut steamCmd_Root = PathBuf::from(env::current_exe().unwrap().parent().unwrap());
    steamCmd_Root.push("steamcmd/");
    let mut steamCmd = steamCmd_Root.clone();
    steamCmd.push(STEAMCMD_EXENAME);
    match steamCmd_Root.exists() {
        true => {}
        false => {
            fs::create_dir(&steamCmd_Root).unwrap();
        }
    }
    match steamCmd.exists() {
        true => {
            println!("Steamcmd executable located.");
        }
        false => {
            let requestData = get(STEAMCMD_DOWNLOADURL).unwrap();
            let content_br = BufReader::new(requestData);
            let tarfile = GzDecoder::new(content_br);
            let mut archive = Archive::new(tarfile);
            archive.unpack(steamCmd_Root);
        }
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![helpme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
