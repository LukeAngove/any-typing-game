use xdotool::desktop::{activate_window,get_active_window};
use xdotool::command::options::{KeyboardOption, SearchOption, SyncOption};
use xdotool::optionvec::OptionVec;
use xdotool::window::search;
use xdotool::keyboard::{send_key_down, send_key_up};
use xdotool::misc::sleep;
use xdotool::option_vec;

pub struct XdoState {
    current : String,
    target : String,
}

impl XdoState {
    pub fn new(window_name : &String) -> XdoState {
        let toutput = search(window_name, option_vec![SearchOption::Name]);
        let mut target = String::from_utf8(toutput.stdout).unwrap();
        target.pop(); // Remove newline

        let coutput = get_active_window();
        let mut current = String::from_utf8(coutput.stdout).unwrap();
        current.pop(); // Remove newline

        XdoState {
            current,
            target,
        }
    }

    pub fn send_key(&self, keys : &String) {
        activate_window(&self.target, option_vec![SyncOption::Sync]);
        send_key_down(keys, option_vec![KeyboardOption::ClearModifiers]);
        sleep(0.1);
        send_key_up(keys, option_vec![KeyboardOption::ClearModifiers]);
        activate_window(&self.current, OptionVec::new());
    }
}