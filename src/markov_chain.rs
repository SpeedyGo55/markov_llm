use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct MarkovChain {
    pub state_map: HashMap<String, Vec<String>>,
    #[serde(skip)]
    rng: rand::rngs::ThreadRng,
    state_size: usize,
}

impl Clone for MarkovChain {
    fn clone(&self) -> Self {
        MarkovChain {
            state_map: self.state_map.clone(),
            state_size: self.state_size,
            rng: self.rng.clone(),
        }
    }
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            state_map: HashMap::new(),
            state_size: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn train(&mut self, data: Vec<String>, state_size: usize) {
        self.state_size = state_size;
        let mut state_map = self.state_map.clone();
        for sentence in data {
            let words = sentence.split_whitespace().collect::<Vec<&str>>();
            for (index, word) in words.iter().enumerate() {
                let mut state = String::new();
                if index + state_size >= words.len() {
                    break;
                }
                let next_state = words[index + state_size].to_string();
                for i in 0..state_size {
                    if i == 0 {
                        state.push_str(word);
                    } else {
                        state.push_str(" ");
                        state.push_str(words[index + i]);
                    }
                }
                let transition_states = state_map.get(&state).cloned();
                match transition_states {
                    Some(mut transitions) => {
                        transitions.push(next_state.clone());
                        state_map.insert(state.clone(), transitions.clone());
                    }
                    None => {
                        state_map.insert(state.clone(), vec![next_state.clone()]);
                    }
                }
            }
        }
        self.state_map = state_map;
    }

    pub fn generate(&mut self, length: usize) -> String {
        if self.state_size == 0 {
            return String::new();
        }
        let mut sentence = vec![String::new()];
        let mut current_state = self.get_sentence_start_state().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        while sentence.len() < length {
            let next_word = self.get_next_state(&current_state);
            match next_word {
                Some(word) => {
                    sentence.push(current_state[0].to_string());
                    current_state = current_state[1..].to_vec();
                    current_state.push(word);
                }
                None => {
                    let mut i = 0;
                    while i < self.state_size && sentence.len() < length {
                        sentence.push(current_state[i].to_string());
                        i += 1;
                    }
                    let last_word = sentence.pop().unwrap();
                    sentence.push(last_word + ".");
                    current_state = self.get_sentence_start_state().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                }
            }
        }
        sentence.join(" ")
    }

    pub fn save(&self, path: &str) {
        let serialized = serde_json::to_string(&self).unwrap();
        std::fs::write(path, serialized).expect("Could not write to file");
    }

    pub fn load(path: &str) -> MarkovChain {
        let serialized = std::fs::read_to_string(path).expect("Could not read file");
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn complete(&mut self, sentence: String, min_len: usize) -> String {
        let mut sentence = sentence.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        if self.state_size > sentence.len() {
            return String::new();
        }
        let mut current_state = sentence[sentence.len() - self.state_size..].to_vec();
        for _ in 0..self.state_size {
            sentence.pop();
        }
        loop {
            let next_word = self.get_next_state(&current_state);
            match next_word {
                Some(word) => {
                    sentence.push(current_state[0].to_string());
                    current_state = current_state[1..].to_vec();
                    current_state.push(word);
                }
                None => {
                    let mut i = 0;
                    while i < self.state_size && sentence.len() < 100 {
                        sentence.push(current_state[i].to_string());
                        i += 1;
                    }
                    let last_word = sentence.pop().unwrap();
                    sentence.push(last_word + ".");
                    if sentence.len() >= min_len {
                        break;
                    }
                    current_state = self.get_sentence_start_state().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                }
            }
        }
        sentence.join(" ")
    }

    fn get_sentence_start_state(&mut self) -> String {
        let states = self.state_map.keys().cloned().collect::<Vec<String>>();
        let start_states = states.iter().filter(|x| x.chars().next().unwrap().is_uppercase()).map(|x| x.to_string()).collect::<Vec<String>>();
        let random_state = self.rng.gen_range(0..start_states.len());
        start_states[random_state].to_string()
    }
    fn get_next_state(&mut self, state: &Vec<String>) -> Option<String> {
        let key = state.join(" ");
        let transitions = self.state_map.get(&key).cloned();
        match transitions {
            Some(transition) => {
                let random_transition = self.rng.gen_range(0..transition.len());
                Some(transition[random_transition].to_string())
            }
            None => {
                None
            }
        }
    }
}