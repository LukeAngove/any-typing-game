use std::io::stdin;
use std::env;

mod xdo;
use xdo::XdoState;

mod typing;
use typing::ChoicesState;

mod configuration;
use configuration::Configuration;

fn main_loop(state : &mut ChoicesState, xstate : &XdoState) -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();

    state.print_data();

    stdin().read_line(&mut s)?;
    s.pop(); // Remove newline.

    let keys = state.consume(&s);

    match keys {
        Some(k) => {xstate.send_key(&k);},
        None => {println!("No matching key found for: {}", s);},
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Vec<String> = env::args().collect();
    
    let app = &args[1];
    let xstate = XdoState::new(&app);

    let conf_file = "config.yml".to_string();
    let conf = Configuration::new(&conf_file)?;
    let mut state = ChoicesState::new(conf, 10);
    
    loop {
        main_loop(&mut state, &xstate)?;
    }
}
