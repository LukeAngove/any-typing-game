#![feature(never_type)]

use std::env;

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
    let args : Vec<String> = env::args().collect();
    
    let app = &args[1];

    let conf_file = "config.yml".to_string();
    let conf = Configuration::new(&conf_file)?;
    let doer = Doer::new(conf, app);

    //let mut ui = TextDisplay::new(doer);
    //let mut ui = Gui::new(doer);
    let mut ui = TUI::new(doer);
    
    ui.main_loop()?
}
