use crate::typing::ChoicesState;
use crate::configuration::Configuration;
use crate::xdo::XdoState;

pub struct Doer {
    pub state : ChoicesState,
    xstate : XdoState,
}

impl Doer {
    pub fn new(conf : Configuration, target : &str) -> Self {
        let state = ChoicesState::new(conf, 3);
        let xstate = XdoState::new(target);

        Doer {
            state,
            xstate,
        }
    }
    
    pub fn check_and_do(&mut self, input : &str) -> Result<(), Box<dyn std::error::Error>> {
        let trimmed = input.trim(); // Remove newline from read_line and any leading/trailing whitespace.
        let keys = self.state.consume(trimmed);

        match keys {
            Some(k) => {self.xstate.send_key(&k);},
            None => {println!("No matching key found for: {}", trimmed);},
        }

        Ok(())
    }
}