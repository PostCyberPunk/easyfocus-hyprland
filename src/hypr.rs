use super::types::HyprWin;
#[allow(unused_imports)]
use hyprland::data::{Client, Clients, FullscreenMode, Monitor, Workspace};
use hyprland::prelude::*;

pub fn get_win_workspace() -> Vec<HyprWin> {
    // TODO: need a better way to get active worksapce
    let active_window = Client::get_active().unwrap().unwrap();
    //TODO: add error handling for fullscreen?
    //Check if active window is fullscreen, if so, exit
    if active_window.fullscreen == FullscreenMode::Fullscreen {
        panic!("Fullscreen window detected,exit");
    }

    let active_workspace_id = active_window.workspace.id;
    let wins: Vec<HyprWin> = Clients::get()
        .unwrap()
        .into_iter()
        .filter(|w| w.workspace.id == active_workspace_id)
        .map(HyprWin::from)
        .collect();
    wins
}
pub fn get_reseverd() -> (i32, i32) {
    let m = Monitor::get_active().expect("cant find monitor?").reserved;
    (m.0 as i32, m.1 as i32)
}

// pub fn my_func() {
//     println!("{:?}", get_win_workspace());
// }
