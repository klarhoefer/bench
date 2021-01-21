
use std::io;
use std::io::prelude::*;
use std::fs;
use std::path::Path;


pub struct Words {
    payload: Vec<String>,
}

impl Words {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let rdr = io::BufReader::new(file);
        let payload = rdr.lines().map(|line| line.unwrap()).collect();
        Ok(Words { payload })
    }

    pub fn iter(&self) -> WordIter {
        WordIter { words: self, current: 0 }
    }

    pub fn content(&self) -> &[String] {
        &self.payload
    }

    pub fn count(&self) -> usize {
        self.payload.len()
    }
}


pub struct WordIter<'a> {
    words: &'a Words,
    current: usize,
}

impl<'a> Iterator for WordIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.words.payload.len() == 0 {
            None
        } else {
            let current = self.current;
            self.current += 1;
            if self.current == self.words.payload.len() {
                self.current = 0;
            }
            Some(&self.words.payload[current])
        }
    }
}
