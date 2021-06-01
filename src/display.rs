use std::collections::{HashMap, VecDeque};

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
