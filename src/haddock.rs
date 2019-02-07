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
            dict: Haddock::load_dict(file),
            length: length,
        }
    }

    pub fn generate(&self) -> Option<String> {
        if self.length == 0 {
            return None;
        }

        let word_len = (0.75 * self.length as f32).floor() as usize;

        let mut word_1 = String::new();
        let mut word_2 = String::new();

        while (word_1.chars().count() + word_2.chars().count()) != word_len {
            word_1 = self.word();
            word_2 = self.word();
        }

        let filler = self.filler(self.length - word_len);

        let mut pass = String::new();
        pass.push_str(&word_1);
        pass.push_str(&filler);
        pass.push_str(&word_2);

        return Some(pass);
    }

    fn load_dict(file: &str) -> Vec<String> {
        let words = fs::read_to_string(file)
            .expect("Something went wrong reading the file");

        let mut dict : Vec<String> = words.lines()
            .map(str::trim)
            .filter(|w| !w.is_empty())
            .map(|w| w.to_owned())
            .collect();

        dict.sort_unstable_by(|a, b| a.chars().count().cmp(&b.chars().count()));

        return dict;
    }

    fn word(&self) -> String {
        let mut rng = thread_rng();
        return self.dict.choose(&mut rng).unwrap().to_owned();
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
