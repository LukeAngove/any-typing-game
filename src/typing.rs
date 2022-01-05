use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};

use crate::configuration::Configuration;
use crate::configuration::DisplayableTypeable;
use crate::configuration::{DictEntry, HardCopyable, HardDictEntry};

pub struct ChoicesState {
    pub conf: Configuration,
    rng: ThreadRng,
    pub choices: HashMap<String, HardDictEntry>,
    pub queues: HashMap<String, VecDeque<HardDictEntry>>,
}

impl ChoicesState {
    pub fn new(conf: Configuration, queue_size: usize) -> Self {
        let mut choices = HashMap::<String, HardDictEntry>::new();
        let mut queues = HashMap::<String, VecDeque<HardDictEntry>>::new();
        let mut rng = thread_rng();

        for d in conf.dictionaries.keys() {
            let mut this_queue = VecDeque::new();
            this_queue.reserve(queue_size);
            fill_queue(
                &choices,
                &mut this_queue,
                &conf.dictionaries[d].iter().collect(),
                &mut rng,
            );
            queues.insert((*d).clone(), this_queue);
        }

        for (key, dict) in &conf.keys {
            let mut queue = queues.get_mut(dict).unwrap();
            let new = queue.pop_back().unwrap();
            choices.insert(key.clone(), new);
            fill_queue(
                &choices,
                &mut queue,
                &conf.dictionaries[dict].iter().collect(),
                &mut rng,
            );
        }

        ChoicesState {
            conf: conf,
            rng: rng,
            choices: choices,
            queues: queues,
        }
    }

    pub fn consume(&mut self, s: &str) -> Option<String> {
        let mut res: Option<String> = None;

        for (k, v) in &self.choices {
            if s == v.matchable() {
                res = Some(k.clone());
                break;
            }
        }

        match res {
            Some(key) => {
                let queue_id = &self.conf.keys[&key];
                let mut queue = self.queues.get_mut(queue_id).unwrap();
                let dict = &self.conf.dictionaries[queue_id]
                    .iter()
                    .collect::<Vec<DictEntry>>();

                let new = queue.pop_front().unwrap();
                self.choices.insert(key.clone(), new);
                fill_queue(&self.choices, &mut queue, &dict, &mut self.rng);
                Some(key)
            }
            None => None,
        }
    }
}

fn fill_queue<'a>(
    choices: &HashMap<String, HardDictEntry>,
    this_queue: &mut VecDeque<HardDictEntry>,
    this_dict: &Vec<DictEntry<'a>>,
    rng: &mut ThreadRng,
) {
    while this_queue.len() < this_queue.capacity() {
        // Revisit this, as capacity may be more than requested, just not less.
        loop {
            // TODO There's an infinite loop here if we can't find a non-conflicing matchable.
            let new = this_dict.choose(rng).unwrap();

            if !(choices.values().into_iter().any(|v| {
                let min = min(v.matchable().chars().count(), new.matchable().chars().count());
                v.matchable().as_str().chars().take(min).eq((*new).matchable().as_str().chars().take(min))
            }) || this_queue.iter().any(|v| {
                let min = min(v.matchable().chars().count(), new.matchable().chars().count());
                v.matchable().as_str().chars().take(min).eq((*new).matchable().as_str().chars().take(min))
            })) {
                this_queue.push_back(new.hard_copy());
                break;
            }
        }
    }
}
