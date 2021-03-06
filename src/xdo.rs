use xdotool::command::options::{KeyboardOption, SearchOption, SyncOption};
use xdotool::desktop::{activate_window, get_active_window};
use xdotool::keyboard::{send_key, send_key_down, send_key_up};
use xdotool::misc::sleep;
use xdotool::option_vec;
use xdotool::optionvec::OptionVec;
use xdotool::window::search;

pub struct XdoState {
    current: String,
    target: String,
    active: bool,
}

impl XdoState {
    pub fn new(window_name: &str, window_index: usize, active: bool) -> XdoState {
        let toutput = search(window_name, option_vec![SearchOption::Name]);
        let mut target = String::from_utf8(toutput.stdout).unwrap();
        target = target.split('\n').collect::<Vec<&str>>()[window_index].to_string();
        println!("Targeting: {}", target);

        let coutput = get_active_window();
        let mut current = String::from_utf8(coutput.stdout).unwrap();
        current.pop(); // Remove newline

        XdoState {
            current,
            target,
            active,
        }
    }

    pub fn send_key(&self, keys: &str) {
        if self.active {
            self.active_send_key(keys);
        } else {
            self.inactive_send_key(keys);
        }
    }

    pub fn send_key_down(&self, keys: &str) {
        if self.active {
            activate_window(&self.target, option_vec![SyncOption::Sync]);
        }
        send_key_down(
            keys,
            option_vec![
                KeyboardOption::ClearModifiers,
                KeyboardOption::Window(self.target.clone()),
            ],
        );
        if self.active {
            activate_window(&self.current, OptionVec::new());
        }
    }

    pub fn send_key_up(&self, keys: &str) {
        if self.active {
            activate_window(&self.target, option_vec![SyncOption::Sync]);
        }
        send_key_up(
            keys,
            option_vec![
                KeyboardOption::ClearModifiers,
                KeyboardOption::Window(self.target.clone()),
            ],
        );
        if self.active {
            activate_window(&self.current, OptionVec::new());
        }
    }

    pub fn inactive_send_key(&self, keys: &str) {
        send_key(
            keys,
            option_vec![
                KeyboardOption::ClearModifiers,
                KeyboardOption::Window(self.target.clone()),
            ],
        );
    }

    pub fn active_send_key(&self, keys: &str) {
        activate_window(&self.target, option_vec![SyncOption::Sync]);
        send_key_down(keys, option_vec![KeyboardOption::ClearModifiers]);
        sleep(0.1);
        send_key_up(keys, option_vec![KeyboardOption::ClearModifiers]);
        activate_window(&self.current, OptionVec::new());
    }
}
