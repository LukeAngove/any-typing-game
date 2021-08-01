use std::{
    collections::{HashMap, VecDeque},
    io,
    convert::TryInto,
};
use crate::event::{Event, Events};
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph},
    Terminal,
};
use crate::ui_trait::UI;
use crate::backend::Doer;

pub trait Renderable {
    fn render(&self, fmt : &str) -> String;
}

impl Renderable for HashMap<String,String> {
    fn render(&self, fmt : &str) -> String {
        let mut res = fmt.to_string();

        for (k,v) in self {
            let to_rep = format!("{{{}}}", k);
            let replacement = format!("{}", v);
            res = res.replace(&to_rep, &replacement);
        }
        
        res
    }
}

impl Renderable for HashMap<String,VecDeque<String>> {
    fn render(&self, fmt : &str) -> String {
        let mut res = fmt.to_string();

        for (k,v) in self {
            let to_rep = format!("{{q:{}}}", k);
            let (s1, s2) = v.as_slices();
            let str1 = s1.join(" ");
            let str2 = s2.join(" ");
            let replacement = format!("{}", [str1, str2].join(" "));
            res = res.replace(&to_rep, &replacement);
        }
        
        res
    }
}

pub struct TUI {
    doer : Doer,
}

impl TUI {
    fn print_data(&self) -> String {
        let res = self.doer.state.choices.render(&self.doer.state.conf.layout);
        let res = self.doer.state.queues.render(&res);
        res
    }

    fn send_key(&mut self, key : &str, held : &mut HashMap<String, u16>) {
        let held_counter_init = 5;
        if !held.contains_key(key) {
            self.doer.mapped_direct_send_down(key);
        }
        held.insert(key.to_string(), held_counter_init);
    }
}

impl UI for TUI {
    fn new(doer : Doer) -> Self {
        TUI {
            doer
        }
    }
    
    fn main_loop(&mut self) -> Result<!, Box<dyn std::error::Error>> {
        let events = Events::new();
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let rows = self.print_data().matches('\n').count().try_into()?;
        let mut terminal = Terminal::new(backend)?;
        let mut string_state = "".to_string();
        // Used to check if we are entering keys in a row,
        // so we don't get just the start of words from Plover; the
        // whole word will be typed before trying to match it.
        let mut key_sequence;
        let mut held = HashMap::<String, u16>::new();

        loop {
            key_sequence = false;
            let mut direct_send = false;

            if let Event::Input(input) = events.next()? {
                key_sequence = true;
                match input {
                    Key::Down     => self.send_key("down", &mut held),
                    Key::Left     => self.send_key("left", &mut held),
                    Key::Right    => self.send_key("right", &mut held),
                    Key::Up       => self.send_key("up", &mut held),
                    Key::Home     => self.send_key("home", &mut held),
                    Key::End      => self.send_key("end", &mut held),
                    Key::PageUp   => self.send_key("pageup", &mut held),
                    Key::PageDown => self.send_key("pagedown", &mut held),
                    Key::BackTab  => self.send_key("backtab", &mut held),
                    Key::Delete   => self.send_key("delete", &mut held),
                    Key::Insert   => self.send_key("insert", &mut held),
                    Key::Backspace => { string_state.pop(); },
                    Key::Esc => panic!("Fix this later, just how we quit for now."),
                    Key::Char('\n') => direct_send = true,
                    Key::Char(c) => string_state.push(c),
                    _ => {}
                }
            }

            let mut to_remove = Vec::<String>::new();

            for (k, v) in held.iter_mut() {
                *v -= 1;
                if *v == 0 {
                    to_remove.push(k.clone());
                }
            }

            for i in to_remove {
                self.doer.direct_send_up(&i);
                held.remove(&i);
            }

            // Remove initial spaces for working with Plover. Keep spaces after for mutliple words
            // If there's just a single space, don't strip; we must be trying to send that.
            if string_state.len() > 1 {
                string_state = string_state.trim_start().to_string();
            }

            // Draw after all state changes have happened.
            // This mades the display more in line with what the viewer expects,
            // e.g.: Show the whole matched word as the 'do' occurs.
            let ss_render = format!("Current buffer:\n|{}|", string_state);
            let menu = self.print_data();
            terminal.draw(|f| {
                let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(rows),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
                f.render_widget(Paragraph::new(menu), chunks[0]);
                f.render_widget(Paragraph::new(ss_render), chunks[1]);
            })?;
 
            if direct_send {
                let ss;
                // If there's just a single space, don't strip; we must be trying to send that.
                if string_state.len() > 1 {
                    ss = string_state.trim_end(); // Remove trailing spaces if sending directly.
                } else {
                    ss = &string_state;
                }
                for s in ss.chars() {
                    if s == ' ' {
                        self.doer.direct_send("space")
                    } else {
                        self.doer.direct_send(&s.to_string());
                    }
                }
                string_state = "".to_string();
            } else {
                // Don't run check and do the same cycle as a key, so we can get the whole word first.
                if !key_sequence && self.doer.check_and_do(&string_state) {
                    string_state = "".to_string();
                }
            }
        }
    }
}

