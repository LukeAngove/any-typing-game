use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, VecDeque};

use crate::configuration::Configuration;

pub struct ChoicesState {
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
        println!("{:?}", self.queues);
        println!("{:?}", self.choices);
    }

    pub fn consume(&mut self, s : &String) -> Option<String> {
        let r = self.choices.get(s).clone();
        match r {
            Some(v) => {
                let val = v.clone();
                self.choices.remove(s);

                let queue_id = &self.conf.keys[&val];
                let mut queue = self.queues.get_mut(queue_id).unwrap();
                let dict = &self.conf.dictionaries[queue_id];

                let new = get_from_queue(&mut queue, &self.choices, dict, &mut self.rng);
                self.choices.insert(new, val.clone());
                return Some(val);
            },
            None => {
                println!("No match for: {}", *s);
                return None;
            },
        }
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
                this_queue.push_front(new.clone());
                break;
            }
        }
    }
}
