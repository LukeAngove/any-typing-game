use std::io::stdin;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use serde::{Deserialize};
use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Deserialize)]
struct Configuration {
    dictionaries: HashMap<String, Vec<String>>,
    keys: HashMap<String, String>,
}

struct ChoicesState {
    conf: Configuration,
    rng: ThreadRng,
    choices: HashMap<String, String>,
    queues: HashMap<String, VecDeque<String>>,
}

impl ChoicesState {
    pub fn new(conf : Configuration, queue_size : usize) -> Self {
        let mut choices = HashMap::<String, String>::new();
        let mut queues = HashMap::<String, VecDeque<String>>::new();
        let mut rng = thread_rng();

        for d in conf.dictionaries.keys() {
            let mut this_queue = VecDeque::new();
            this_queue.reserve(queue_size);
            fill_queue(&choices, &mut this_queue, &conf.dictionaries[d], &mut rng);
            queues.insert((*d).clone(), this_queue);
        }
        
        for (key,dict) in &conf.keys {
            let mut queue = queues.get_mut(dict).unwrap();
            let next = get_from_queue(&mut queue, &choices, &conf.dictionaries[dict], &mut rng);
            choices.insert(next, (*key).clone());
        }

        let me = ChoicesState {
            conf: conf,
            rng: rng,
            choices: choices,
            queues: queues,
        };
 
        me
    }

    pub fn print_data(&self) {
        println!("{:?}", self.choices);
    }

    pub fn consume(&mut self, s : &String) {
        let val;
        {
            let r = self.choices.get(s);
            match r {
                Some(v) => {
                    println!("Got: {}", v);
                    val = v.clone();
            },
                None => {
                    println!("No match for: {}", *s);
                    return;
                },
            }
        }

        self.choices.remove(s);

        let queue_id = &self.conf.keys[&val];
        let mut queue = self.queues.get_mut(queue_id).unwrap();
        let dict = &self.conf.dictionaries[queue_id];

        let new = get_from_queue(&mut queue, &self.choices, dict, &mut self.rng);
        self.choices.insert(new, val);
    }
    

}

fn get_from_queue(queue : &mut VecDeque<String>, choices : &HashMap<String, String>, dictionary : &Vec<String>, rng : &mut ThreadRng) -> String {
    let res = queue.pop_back();
    fill_queue(choices, queue, dictionary, rng);
    return res.unwrap(); 
}

fn fill_queue(choices : &HashMap<String, String>, this_queue : &mut VecDeque<String>, this_dict : &Vec<String>, rng : &mut ThreadRng) {
    while this_queue.len() < this_queue.capacity() { // Revisit this, as capacity may be more than requested, just not less.
        loop {
            let new = this_dict.choose(rng).unwrap();
            if choices.get(new) == None && !this_queue.contains(new) {
                this_queue.push_back(new.clone());
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf: Configuration;

    {
        let file = File::open("config.yml")?;
        conf = serde_yaml::from_reader(file)?;
    }
    
    let mut state = ChoicesState::new(conf, 10);
    
    loop {
        let mut s = String::new();

        state.print_data();

        stdin().read_line(&mut s)?;
        s.pop(); // Remove newline.

        state.consume(&s);
    }
}
