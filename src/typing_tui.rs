use std::{
    error::Error,
    collections::{HashMap, VecDeque},
    io,
    convert::TryInto,
};
use crate::event::{Event, Events};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crate::ui_trait::UI;
use crate::backend::Doer;
use std::io::stdin;

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

        loop {
            let ss_render = string_state.clone();
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
            });

            if let Event::Input(input) = events.next()? {
                match input {
                    Key::Char('\n') => {
                        //app.messages.push(app.input.drain(..).collect());
                    }
                    Key::Char(c) => {
                        string_state.push(c);
                    }
                    Key::Backspace => {
                        string_state.pop();
                    }
                    Key::Esc => {
                        panic!("Fix this later, just how we quit for now.");
                    }
                    _ => {}
                }
            }

            if (self.doer.check_and_do(&string_state)?) {
                string_state = "".to_string();
            }
        }
    }
}

