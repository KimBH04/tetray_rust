use std::{env::home_dir, fs::File, io::{Read, Write}, path::PathBuf};

fn get_highscore_path() -> Option<PathBuf> {
    if let Some(mut p) = home_dir() {
        p.push("tetray_highscore.txt");
        Some(p)
    } else {
        None
    }
}

fn read_highscore() -> usize {
    if let Some(p) = get_highscore_path() {
        let path = p.as_path();
        if let Ok(mut file) = File::open(path) {
            let mut s = String::new();
            if let Ok(_) = file.read_to_string(&mut s) {
                if let Ok(highscore) = s.parse() {
                    return highscore
                }
            }
        }
    }
    0
}

fn write_highscore(highscore: usize) {
    if let Some(p) = get_highscore_path() {
        let path = p.as_path();
        if let Ok(mut file) = File::create(path) {
            let s = format!("{highscore}");
            _ = file.write_all(s.as_bytes());
        }
    }
}

pub(super) struct Score {
    current: usize,
    high: usize,
}

impl Drop for Score {
    fn drop(&mut self) {
        write_highscore(self.get_highscore());
    }
}

impl Score {
    pub(super) fn new() -> Self {
        Self {
            current: 0,
            high: read_highscore(),
        }
    }

    pub(super) fn add_score(&mut self, amount: usize) {
        self.current += amount;
    }

    pub(super) fn get_score(&self) -> usize {
        self.current
    }

    pub(super) fn get_highscore(&self) -> usize {
        if self.current > self.high {
            self.current
        } else {
            self.high
        }
    }
}