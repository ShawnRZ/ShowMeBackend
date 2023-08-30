// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod cer;
pub mod game;
pub mod lcu;
pub mod sgp;
pub mod summoner;
pub mod websocket;

use std::error::Error;

use env_logger::Env;
use lcu::match_history::LolMatchHistoryMatchHistoryGame;
use lcu::ranked_stats::RankedStats;
use lcu::summoner::Summoner as LcuSummoner;
use lcu::{match_history::MatchHistory, parameter::Parameter};
use log::{debug, error, info, trace, warn};
use serde_json::Value;
use tauri::Manager;
use tokio::task::JoinSet;
use window_shadows::set_shadow;

#[tauri::command]
async fn get_parameter() -> Result<Parameter, String> {
    debug!("lcu::get_parameter()");
    let lp = Parameter::get().await.map_err(|e| e.to_string())?;
    Ok(lp)
}

#[tauri::command]
async fn refresh_parameter() -> Result<(), String> {
    debug!("lcu::refresh_parameter()");
    Parameter::refresh().await.map_err(|e| e.to_string())?;
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn connect<R: tauri::Runtime>(window: tauri::Window<R>) -> Result<(), String> {
    websocket::connect(window)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_champ_select_session() -> Result<String, String> {
    let res = lcu::request::get_champ_select_session()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res)
}

#[tauri::command]
async fn get_current_summoner() -> Result<LcuSummoner, String> {
    let s = lcu::request::get_current_summoner()
        .await
        .map_err(|e| e.to_string())?;
    Ok(s)
}

#[tauri::command]
async fn get_summoner_by_name(name: String) -> Result<LcuSummoner, String> {
    let mut set: JoinSet<Result<LcuSummoner, Box<dyn Error + Send + Sync>>> = JoinSet::new();
    set.spawn(lcu::request::get_summoner_by_name(name.clone()));
    // let region = lcu::parameter::Parameter::get()
    //     .await
    //     .map_err(|e| e.to_string())?
    //     .region;
    // set.spawn(sgp::request::get_summoner_by_name(name.clone(), region));

    let s = set
        .join_next()
        .await
        .ok_or("出了点问题")?
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    Ok(s)
}

#[tauri::command]
async fn get_match_history_by_puuid(
    puuid: String,
    beg: u64,
    end: u64,
) -> Result<MatchHistory, String> {
    let mut set = JoinSet::new();

    set.spawn(lcu::request::get_match_history(puuid.clone(), beg, end));

    // set.spawn(sgp::request::get_match_history(puuid, beg, end - beg + 1));

    let mh = set
        .join_next()
        .await
        .ok_or("出了点问题")?
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    Ok(mh)
}

#[tauri::command]
async fn get_match_by_id(id: u64) -> Result<LolMatchHistoryMatchHistoryGame, String> {
    let mut set = JoinSet::new();

    set.spawn(lcu::request::get_match_by_id(id));

    // set.spawn(sgp::request::get_match_by_id(id));

    let mh = set
        .join_next()
        .await
        .ok_or("出了点问题")?
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    Ok(mh.games.unwrap().games.unwrap().remove(0))
}

#[tauri::command]
async fn get_ranked_stats(puuid: String) -> Result<RankedStats, String> {
    let r = lcu::request::get_ranked_stats(&puuid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(r)
}

#[tauri::command]
async fn launch_spectate(puuid: String) -> Result<(), String> {
    lcu::request::spectate(&puuid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn test_and_set_cer() -> Result<bool, String> {
    let res = cer::test_and_set().map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
async fn get_summoner_by_id(id: u64) -> Result<LcuSummoner, String> {
    let s = lcu::request::get_summoner_by_id(id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(s)
}

#[tauri::command]
async fn get_gameflow_session() -> Result<Value, String> {
    let s = lcu::request::get_gameflow_session()
        .await
        .map_err(|e| e.to_string())?;
    Ok(s)
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            set_shadow(&window, true).expect("Unsupported platform!");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect,
            get_parameter,
            refresh_parameter,
            get_champ_select_session,
            get_current_summoner,
            get_summoner_by_name,
            get_match_history_by_puuid,
            get_match_by_id,
            get_ranked_stats,
            launch_spectate,
            test_and_set_cer,
            get_summoner_by_id,
            get_gameflow_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
