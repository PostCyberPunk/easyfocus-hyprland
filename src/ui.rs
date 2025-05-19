use hyprland::dispatch::*;
use hyprland::shared::Address;
use std::collections::HashMap;

use gtk4::{gdk, glib};
use gtk4::{prelude::*, Application, CssProvider, EventControllerKey};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::{cli::Args, hypr, types::HyprWin, utils};

//FIX: dont use clone here
fn calculate_geometry(window: &HyprWin, args: Args, reseverd: &(i32, i32)) -> (i32, i32) {
    // TODO: this doesn't work properly with stacked windows

    let rel_x = window.size.0 / 2 + args.label_margin_x.unwrap() - reseverd.0;
    let rel_y = args.label_margin_y.unwrap() - reseverd.1;

    (rel_x + window.at.0, window.at.1 + rel_y)
}

fn switch_window(win_add: &Address) {
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        win_add.clone(),
    )))
    .expect("failed to focus window");
}

fn handle_keypress(key_to_con_id: &HashMap<char, Address>, keyval: &str) {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            if key_to_con_id.contains_key(&c) {
                switch_window(&key_to_con_id[&c]);
            } else {
                panic!("no window found for key: {}", c);
            }
        }
    } else {
        //TODO: handle special keys,like escape ,meh, for now just panic
        panic!("illegal key: {}", keyval);
    }
}

fn build_ui(app: &Application, args: Args) {
    // get windows from hyprland
    let windows = hypr::get_win_workspace();
    // exit if no windows open
    if windows.is_empty() {
        panic!("No windows open");
    }
    let reserved = hypr::get_reseverd();
    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    let app_win = gtk4::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    LayerShell::init_layer_shell(&app_win);
    // app_win.set_title("easyfocus");
    // display it above normal windows
    LayerShell::set_layer(&app_win, Layer::Overlay);

    // receive keyboard events from the compositor
    LayerShell::set_keyboard_mode(&app_win, KeyboardMode::OnDemand);

    // take up the full screen
    LayerShell::set_anchor(&app_win, Edge::Top, true);
    LayerShell::set_anchor(&app_win, Edge::Bottom, true);
    LayerShell::set_anchor(&app_win, Edge::Left, true);
    LayerShell::set_anchor(&app_win, Edge::Right, true);
    let fixed = gtk4::Fixed::new();
    // map keys to window Ids
    let mut key_to_con_id = HashMap::new();

    windows.iter().for_each(|win| {
        let (x, y) = calculate_geometry(win, args.clone(), &reserved);
        let label = gtk4::Label::new(Some(""));
        let letter = chars.next().unwrap();
        key_to_con_id.insert(letter, win.address.clone());
        label.set_markup(&format!("{}", letter));
        fixed.put(&label, x as f64, y as f64);

        // Apply a CSS class to the focused window so it can be styled differently
        if win.focused {
            label.style_context().add_class("focused");
        }
    });

    let key_event_controller = EventControllerKey::new();
    let win_clone = app_win.clone();
    key_event_controller.connect_key_pressed(move |_, key, _, _| {
        let keyval = key.name().unwrap();
        handle_keypress(&key_to_con_id, &keyval);
        win_clone.close();
        gtk4::glib::Propagation::Stop
    });

    app_win.add_controller(key_event_controller);

    let provider = CssProvider::new();
    let css = utils::args_to_css(&args);
    provider.load_from_data(&css);

    app_win
        .style_context()
        .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    app_win.set_child(Some(&fixed));
    app_win.present();
}

pub fn run_ui(args: Args) {
    let app = Application::builder()
        .application_id("com.github.pcp.easyfocus-hyprland")
        .build();

    // let args_clone = args.clone();
    // app.connect_startup(move |_| load_css(args_clone.clone()));
    app.connect_activate(move |app| {
        build_ui(app, args.clone());
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
