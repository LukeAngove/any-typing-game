use xdotool::desktop::{activate_window,get_active_window};
use xdotool::command::options::{KeyboardOption, SearchOption, SyncOption};
use xdotool::optionvec::OptionVec;
use xdotool::window::search;
use xdotool::keyboard::send_key;
use xdotool::option_vec;

pub struct XdoState {
    current : String,
    target : String,
}

impl XdoState {
    pub fn new(window_name : &String) -> XdoState {
        let toutput = search(window_name, option_vec![SearchOption::Name]);
        println!("{:?}", toutput);
        let mut target = String::from_utf8(toutput.stdout).unwrap();
        target.pop();
        println!("{:?}", target);

        let coutput = get_active_window();
        println!("{:?}", coutput);
        let mut current = String::from_utf8(coutput.stdout).unwrap();
        current.pop();
        println!("{:?}", current);

        XdoState {
            current,
            target,
        }
    }

    pub fn send_key(&self, keys : &String) {
        activate_window(&self.target, option_vec![SyncOption::Sync]);
        send_key(keys, option_vec![KeyboardOption::ClearModifiers]);
        activate_window(&self.current, OptionVec::new());
    }
}