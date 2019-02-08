extern crate rand;

use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::fs;

pub struct Haddock {
    dict: Vec<String>,
    length: usize,
}

impl Haddock {
    pub fn new(length: usize, file: &str) -> Haddock {

        Haddock {
            dict: Haddock::load_dict(file).unwrap(),
            length: length,
        }
    }

    pub fn generate(&self) -> Option<String> {
        if self.length == 0 {
            return None;
        }
        if self.length < 8 || self.length > 50 {
            return None;
        }

        let word_len = (0.75 * self.length as f32).floor() as usize;

        let mut word_1 = String::new();
        let mut word_2 = String::new();

        while (word_1.chars().count() + word_2.chars().count()) != word_len {
            word_1 = match self.word() {
                Some(word)  =>  word,
                None        => return None,
            };
            word_2 = match self.word() {
                Some(word)  => word,
                None        => return None,
            }
        }

        let filler = self.filler(self.length - word_len);

        let mut pass = String::new();
        pass.push_str(&word_1);
        pass.push_str(&filler);
        pass.push_str(&word_2);

        return Some(pass);
    }

    fn load_dict(file: &str) -> Result<Vec<String>, &'static str> {
        let words = match fs::read_to_string(file) {
            Ok(words)   => words,
            Err(_)    => return Err("Unable to open wordlist"),
        };

        let dict : Vec<String> = words.lines()
            .map(str::trim)
            .filter(|w| !w.is_empty())
            .map(|w| w.to_owned())
            .collect();

        return Ok(dict);
    }

    fn word(&self) -> Option<String> {
        let mut rng = thread_rng();
        match self.dict.choose(&mut rng) {
            Some(word)  =>  return Some(word.to_owned()),
            None        =>  return None,
        }
    }

    fn filler(&self, length: usize) -> String {
        let chars = ["0","1","2","3","4","5","6","7","8","9","0","`","~","!",
            "@","#","$","%","^","&","*","(",")","-","_","=","+","[","{","]","}",
            "\\","|",";",":","'","\"",",","<",".",">","/","?"];

        let mut rng = thread_rng();

        return chars.choose_multiple(&mut rng, length)
            .map(|&x| x.to_owned())
            .collect();
    }
}
