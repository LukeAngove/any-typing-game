use crate::typing::ChoicesState;
use crate::configuration::Configuration;
use crate::xdo::XdoState;

pub struct Doer {
    pub state : ChoicesState,
    xstate : XdoState,
}

impl Doer {
    pub fn new(conf : Configuration, target : &str, window_index : usize, active : bool) -> Self {
        let state = ChoicesState::new(conf, 3);
        let xstate = XdoState::new(target, window_index, active);

        Doer {
            state,
            xstate,
        }
    }
    
    pub fn check_and_do(&mut self, input : &str) -> bool {
        let keys = self.state.consume(input);

        match keys {
            Some(k) => {
                self.xstate.send_key(&k);
                true
            },
            None => {
                false
            },
        }
    }

    pub fn direct_send(&mut self, input : &str) {
        self.xstate.send_key(input);
    }
}