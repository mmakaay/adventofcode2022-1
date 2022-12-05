#[macro_use] extern crate scan_fmt;
use std::time::Instant;

#[derive(Clone)]
struct Stacks(Vec<Vec<u8>>);

impl Stacks {
    fn parse_stacks<'a>(mut lines: impl Iterator<Item = &'a str>) -> Stacks {
        let mut stacks = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.as_bytes();
            if line[1] == b'1' {
                lines.next();
                break;
            }
            let mut idx = 0;
            while idx * 4 + 1 < line.len() {
                let c = line[idx * 4 + 1];
                if stacks.len() <= idx {
                    stacks.push(Vec::new());
                }
                if c >= b'A' && c <= b'Z' {
                    stacks[idx].push(c);
                }
                idx += 1;
            }
        }
        stacks.iter_mut().for_each(|s| s.reverse());
        Stacks(stacks)
    }

    // Prints the stacks flipped 90 degrees clockwise.
    #[allow(dead_code)]
    fn debug(&self) {
        println!("=== stacks ===");
        for s in &self.0 {
            print!("[");
            for c in s {
                print!("{}", *c as char);
            }
            println!("]\n");
        }
    }

    fn cratemover_9000<'a>(&mut self, lines: impl Iterator<Item = &'a str>) {
        type U = usize;
        for cmd in lines {
            let (num, from, to) = scan_fmt!(cmd, "move {} from {} to {}", U, U, U).unwrap();
            for _ in 0 .. num {
                let c = self.0[from - 1].pop().unwrap();
                self.0[to - 1].push(c);
            }
        }
    }

    fn cratemover_9001<'a>(&mut self, lines: impl Iterator<Item = &'a str>) {
        type U = usize;
        for cmd in lines {
            let (num, from, to) = scan_fmt!(cmd, "move {} from {} to {}", U, U, U).unwrap();
            let i = self.0[from-1].len() - num;
            let c = self.0[from-1].split_off(i);
            self.0[to - 1].extend_from_slice(&c);
        }
    }

    fn top_crates(&self) -> String {
        self.0.iter().map(|s| s[s.len()-1] as char).collect()
    }
}

pub fn part1_2(input: &str) {
    let now = Instant::now();
    let mut lines = input.trim_end().split("\n");
    let mut stacks = Stacks::parse_stacks(&mut lines);
    println!("day-05: parsing: {:?}", now.elapsed());

    let now = Instant::now();
    let mut sclone = stacks.clone();
    let mut lclone = lines.clone();
    sclone.cratemover_9000(&mut lclone);
    println!("day-05: part1: {}", sclone.top_crates());
    println!("day-05: part1: {:?}", now.elapsed());

    let now = Instant::now();
    stacks.cratemover_9001(&mut lines);
    println!("day-05: part2: {}", stacks.top_crates());
    println!("day-05: part2: {:?}", now.elapsed());
}
