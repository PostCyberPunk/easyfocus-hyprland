use hyprland::dispatch::*;
use hyprland::shared::Address;
use std::collections::HashMap;
use std::sync::Arc;

use gtk::{prelude::*, Application, CssProvider, StyleContext};

use crate::{cli::Args,hypr, types::HyprWin, utils};

//FIX: dont use clone here
fn calculate_geometry(window: &HyprWin,args: Arc<Args>) -> (i32, i32) {
    // TODO: this doesn't work properly with stacked windows

    let rel_x = window.size.0 / 2 + args.label_margin_x.unwrap();
    let rel_y = args.label_margin_y.unwrap();

    (rel_x + window.at.0, window.at.0 + rel_y)
}

fn handle_keypress(key_to_con_id: &HashMap<char,Address>, keyval: &str) {
    if keyval.len() == 1 {
        // we can unwrap because the keyval has one character
        let c = keyval.chars().next().unwrap();
        if c.is_alphabetic() && c.is_lowercase() {
            let win_add = key_to_con_id[&c].clone();
            Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(win_add))).expect("failed to focus window");
        }
    }
}

fn build_ui(app: &Application, args: Arc<Args>) {
    // get windows from sway
    let windows = hypr::get_win_workspace();
    let letters = args.chars.clone().expect("Some characters are required");
    let mut chars = letters.chars();

    // exit if no windows open
    if windows.is_empty() {
        panic!("No windows open");
    }

    let window = gtk::ApplicationWindow::new(app);

    // before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);
    // display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // receive keyboard events from the compositor
    gtk_layer_shell::set_keyboard_mode(&window, gtk_layer_shell::KeyboardMode::Exclusive);

    // take up the full screen
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

    let fixed = gtk::Fixed::new();
    // map keys to window Ids
    let mut key_to_con_id = HashMap::new();

    windows.iter().enumerate().for_each(|(_idx, window)| {
        let (x, y) = calculate_geometry(window,args.clone());
        let label = gtk::Label::new(Some(""));
        let letter = chars.next().unwrap();
        key_to_con_id.insert(letter, window.address.clone());
        label.set_markup(&format!("{}", letter));
        fixed.put(&label, x, y);

        // Apply a CSS class to the focused window so it can be styled differently
        // if window.focused {
        //     label.style_context().add_class("focused");
        // }
    });

    window.connect_key_press_event(move |window, event| {
        let keyval = event
            .keyval()
            .name()
            .expect("the key pressed does not have a name?");
        handle_keypress(&key_to_con_id, &keyval);
        window.close();
        Inhibit(false)
    });

    window.add(&fixed);
    window.show_all();
}

fn load_css(args: Arc<Args>) {
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

pub fn run_ui(args: Arc<Args>) {
    let app = Application::builder()
        .application_id("com.github.edzdez.sway-easyfocus")
        .build();

    let args_clone = args.clone();
    app.connect_startup(move |_| load_css(args_clone.clone()));
    app.connect_activate(move |app| {
        build_ui(app, args.clone());
    });

    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
