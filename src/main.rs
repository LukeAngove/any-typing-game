#![feature(never_type)]

use clap::{Arg, App};

mod xdo;
mod typing;
mod gui;
use gui::Gui;

mod configuration;
use configuration::Configuration;

mod ui_trait;
use ui_trait::UI;

mod display;
use display::TextDisplay;

mod backend;
use backend::Doer;

mod typing_tui;
use typing_tui::TUI;

mod event;

fn main() -> Result<!, Box<dyn std::error::Error>> {
    let args = App::new("Typing to input")
        .author("Luke Angove <luke.angove@gmail.com>")
        .about("Converts typed into an alternate input (mostly for practicing stenography)")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Configuration for world list mappings/conversions"))
        .arg(Arg::with_name("window_title")
            .required(true)
            .index(1)
            .help("Substring of the window title to target with converted input"))
        .arg(Arg::with_name("window_index")
            .short("w")
            .long("window_index")
            .takes_value(true)
            .help("Index of the window to target if there are multiple matches, defaults to 0"))
        .arg(Arg::with_name("user interface")
            .short("i")
            .long("interface")
            .takes_value(true)
            .help("Name of the interface to use"))
        .arg(Arg::with_name("active_window")
            .short("a")
            .long("active")
            .help("Activate window when sending keys, rather that running in the background")
        )
        .get_matches();
    
    let app = args.value_of("window_title").unwrap();
    let window_index : usize = args.value_of("window_index").unwrap_or("0").parse()?;
    let conf_file = args.value_of("config").unwrap_or("config.yml");
    let interface = args.value_of("user interface").unwrap_or("tui");
    let active = args.is_present("active_window");

    let conf = Configuration::new(conf_file)?;
    let doer = Doer::new(conf, app, window_index, active);

    let mut ui : Box<dyn UI>;
    println!("Interface: {}", interface);
    match interface {
        "gui" => { ui = Box::new(Gui::new(doer)); }
        "tui" => { ui = Box::new(TUI::new(doer)); }
        "text" => { ui = Box::new(TextDisplay::new(doer)); }
        _ => { panic!("Invalid UI!"); }
    }
    
    ui.main_loop()?
}
