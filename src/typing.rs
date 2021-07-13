use rand::thread_rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::collections::{HashMap, VecDeque};
use std::cmp::min;

use crate::configuration::Configuration;

pub struct ChoicesState {
    pub conf: Configuration,
    rng: ThreadRng,
    pub choices: HashMap<String, String>,
    pub queues: HashMap<String, VecDeque<String>>,
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
            let new = queue.pop_back().unwrap();
            choices.insert(key.clone(), new);
            fill_queue(&choices, &mut queue, &conf.dictionaries[dict], &mut rng);
        }

        ChoicesState {
            conf: conf,
            rng: rng,
            choices: choices,
            queues: queues,
        }
    }

    pub fn consume(&mut self, s : &str) -> Option<String> {
        let mut res : Option<String> = None;

        for (k,v) in &self.choices {
            if s == v {
                res = Some(k.clone());
                break;
            }
        }
 
        match res {
            Some(key) => {
                let queue_id = &self.conf.keys[&key];
                let mut queue = self.queues.get_mut(queue_id).unwrap();
                let dict = &self.conf.dictionaries[queue_id];

                let new = queue.pop_front().unwrap();
                self.choices.insert(key.clone(), new);
                fill_queue(&self.choices, &mut queue, dict, &mut self.rng);
                Some(key)
            },
            None => {
                None
            },
        }
    }
}

fn fill_queue(choices : &HashMap<String, String>, this_queue : &mut VecDeque<String>, this_dict : &Vec<String>, rng : &mut ThreadRng) {
    while this_queue.len() < this_queue.capacity() { // Revisit this, as capacity may be more than requested, just not less.
        loop {
            let new = this_dict.choose(rng).unwrap();

            if !(
                    choices.values().into_iter().any(|v| { let min = min(v.len(), new.len()); v[0..min] == (*new).as_str()[0..min] }) ||
                    this_queue.iter().any(|v| { let min = min(v.len(), new.len()); v[0..min] == (*new).as_str()[0..min] })
                ) {
                this_queue.push_back(new.clone());
                break;
            }
        }
    }
}
