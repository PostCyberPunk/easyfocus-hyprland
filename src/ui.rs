use hyprland::dispatch::*;
use hyprland::shared::Address;
use std::collections::HashMap;

use gtk::{prelude::*, Application, CssProvider, StyleContext};

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
            switch_window(&key_to_con_id[&c]);
        }
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

    let app_win = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&app_win);
    // app_win.set_title("easyfocus");
    // display it above normal windows
    gtk_layer_shell::set_layer(&app_win, gtk_layer_shell::Layer::Overlay);

    // receive keyboard events from the compositor
    gtk_layer_shell::set_keyboard_mode(&app_win, gtk_layer_shell::KeyboardMode::OnDemand);

    // take up the full screen
    gtk_layer_shell::set_anchor(&app_win, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&app_win, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&app_win, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&app_win, gtk_layer_shell::Edge::Right, true);
    let fixed = gtk::Fixed::new();
    // map keys to window Ids
    let mut key_to_con_id = HashMap::new();

    windows.iter().for_each(|win| {
        let (x, y) = calculate_geometry(win, args.clone(), &reserved);
        let label = gtk::Label::new(Some(""));
        let letter = chars.next().unwrap();
        key_to_con_id.insert(letter, win.address.clone());
        label.set_markup(&format!("{}", letter));
        fixed.put(&label, x, y);

        // Apply a CSS class to the focused window so it can be styled differently
        if win.focused {
            label.style_context().add_class("focused");
        }
    });

    app_win.connect_key_press_event(move |win, event| {
        let keyval = event
            .keyval()
            .name()
            .expect("the key pressed does not have a name?");
        handle_keypress(&key_to_con_id, &keyval);
        win.close();
        Inhibit(false)
    });

    app_win.add(&fixed);
    app_win.show_all();
}

fn load_css(args: Args) {
    let provider = CssProvider::new();
    provider
        .load_from_data(utils::args_to_css(&args).as_bytes())
        .expect("failed to load css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_screen(
        // we can unwrap because there should be a default screen
        &gtk::gdk::Screen::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn run_ui(args: Args) {
    let app = Application::builder()
        .application_id("com.github.pcp.easyfocus-hyprland")
        .build();

    let args_clone = args.clone();
    app.connect_startup(move |_| load_css(args_clone.clone()));
    app.connect_activate(move |app| {
        build_ui(app, args.clone());
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
