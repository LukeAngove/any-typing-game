use std::collections::{HashMap, VecDeque};
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

pub struct TextDisplay {
    doer : Doer,
}

impl TextDisplay {
    fn print_data(&self) {
        let res = self.doer.state.choices.render(&self.doer.state.conf.layout);
        let res = self.doer.state.queues.render(&res);
        println!("{}", res);
    }
}

impl UI for TextDisplay {
    fn new(doer : Doer) -> Self {
        TextDisplay {
            doer
        }
    }

    fn main_loop(&mut self) -> Result<!, Box<dyn std::error::Error>> {
        loop {
            self.print_data();

            let mut s = String::new();
            stdin().read_line(&mut s)?;
            let trimmed = s.trim(); // Remove newline from read_line and any leading/trailing whitespace.
            let words = trimmed.split(' '); // Split on whitespace to allow multiple words in a row.

            for w in words {
                self.doer.check_and_do(&w);
            }
        }
    }
}
