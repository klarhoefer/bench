
use std::collections::HashSet;
use std::io;
use std::time;

// mod rnd;

trait Haystack {
    fn find(&self, needle: &str) -> bool;
}

impl Haystack for &[String] {
    fn find(&self, needle: &str) -> bool {
        for s in self.iter() {
            if s == needle {
                return true;
            }
        }
        false
    }
}

impl Haystack for HashSet<String> {
    fn find(&self, needle: &str) -> bool {
        self.contains(needle)
    }
}

mod words;
use words::Words;


fn find_all<H: Haystack>(src: &Words, len: usize, haystack: &H) -> (usize, time::Duration) {
    let t0 = time::Instant::now();
    let mut count = 0;
    for word in src.iter().take(len) {
        if haystack.find(word) {
            count += 1;
        }
    }
    (count, t0.elapsed())
}

fn main() -> io::Result<()> {
    // let r = rnd::Pcg::new(23, 1);
    let wordlist = Words::from_file("words.txt")?;

    let hit_rate = 0.245;
    let words_in_container = (wordlist.count() as f64 * hit_rate) as usize;
    println!("{} words in container", words_in_container);

    let existent_words = &wordlist.content()[..words_in_container];

    let mut hs = HashSet::<String>::new();
    for word in existent_words {
        hs.insert(word.clone());
    }

    let iters_per_run = 10_000_000;
    let mut total_dur0 = time::Duration::new(0, 0);
    let mut total_dur1 = time::Duration::new(0, 0);
    for run in 0..10 {
        let (found0, dur0) = find_all(&wordlist, iters_per_run, &hs);
        println!("Run {}, Hash:  {} found, {:?}", run, found0, dur0);
        let (found1, dur1) = find_all(&wordlist, iters_per_run, &existent_words);
        println!("Run {}, Slice: {} found, {:?}", run, found1, dur1);
        total_dur0 += dur0;
        total_dur1 += dur1;
    }
    println!("Hash {:?} vs. {:?} Slice", total_dur0, total_dur1);

    Ok(())
}
