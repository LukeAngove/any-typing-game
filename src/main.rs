use std::io::stdin;
use std::env;

mod xdo;
use xdo::XdoState;

mod typing;
use typing::ChoicesState;

mod configuration;
use configuration::Configuration;

mod display;

fn main_loop(state : &mut ChoicesState, xstate : &XdoState) -> Result<(), Box<dyn std::error::Error>> {
    state.print_data();

    let mut s = String::new();
    stdin().read_line(&mut s)?;
    let trimmed = s.trim(); // Remove newline from read_line and any leading/trailing whitespace.

    let keys = state.consume(trimmed);

    match keys {
        Some(k) => {xstate.send_key(&k);},
        None => {println!("No matching key found for: {}", trimmed);},
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Vec<String> = env::args().collect();
    
    let app = &args[1];
    let xstate = XdoState::new(&app);

    let conf_file = "config.yml".to_string();
    let conf = Configuration::new(&conf_file)?;
    let mut state = ChoicesState::new(conf, 3);
    
    loop {
        main_loop(&mut state, &xstate)?;
    }
}
