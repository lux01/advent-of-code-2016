mod graph;

use graph::Graph;

use std::convert::{From, Into};
use std::collections::HashSet;
use std::time::SystemTime;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Pair {
    chip: i8,
    gen: i8,
}

impl From<(i8, i8)> for Pair {
    fn from(tuple: (i8, i8)) -> Pair {
        Pair { chip: tuple.0, gen: tuple.1 }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct State {
    lift: i8,
    pairs: Vec<Pair>,
    n: usize,
}

impl State {
    pub fn new<T: Into<Pair>>(pairs: Vec<T>) -> State {
        State::with_floor(pairs, 0)
    }

    pub fn with_floor<T: Into<Pair>>(pairs: Vec<T>, lift: i8) -> State {
        State {
            lift: lift,
            n: pairs.len(),
            pairs: pairs.into_iter().map(Into::into).collect(),
        }
    }

    pub fn is_valid(&self) -> bool {
        // The floor must be between 0 and 3
        let mut valid = self.lift >= 0 && self.lift <= 3;

        // Check that a chip is with its generator or that it's not on the
        // same floor as another generator

        for i in 0 .. self.n {
            if self.pairs[i].chip != self.pairs[i].gen {
                for j in 0 .. self.n {
                    if i == j {
                        continue;
                    } else {
                        valid = valid && self.pairs[i].chip != self.pairs[j].gen;
                    }
                }
            }
        }
        
        valid
    }

    pub fn next_states(&self) -> Vec<State> {
        let mut options = HashSet::new();
        for i in 0..self.n {
            // We can move 1 chip
            if self.lift == self.pairs[i].chip {
                let mut up = self.clone();
                let mut down = self.clone();

                up.pairs[i].chip += 1;
                up.lift += 1;
                
                down.pairs[i].chip -= 1;
                down.lift -= 1;

                options.insert(up);
                options.insert(down);
            }

            // or 1 generator
            if self.lift == self.pairs[i].gen {
                let mut up = self.clone();
                let mut down = self.clone();

                up.pairs[i].gen += 1;
                up.lift += 1;
                
                down.pairs[i].gen -= 1;
                down.lift -= 1;

                options.insert(up);
                options.insert(down);
            }

            // Or a combination of 2 things:
            for j in 0..self.n {
                // There are 3 unique options for moving 2 things:
                // (chip, chip)
                if self.pairs[i].chip == self.lift &&
                    self.pairs[j].chip == self.lift &&
                    i != j
                {
                    let mut up = self.clone();
                    let mut down = self.clone();

                    up.pairs[i].chip += 1;
                    up.pairs[j].chip += 1;
                    up.lift += 1;

                    down.pairs[i].chip -= 1;
                    down.pairs[j].chip -= 1;
                    down.lift -= 1;

                    options.insert(up);
                    options.insert(down);
                }

                // (chip, gen)
                if self.pairs[i].chip == self.lift &&
                    self.pairs[j].gen == self.lift
                {
                    let mut up = self.clone();
                    let mut down = self.clone();

                    up.pairs[i].chip += 1;
                    up.pairs[j].gen += 1;
                    up.lift += 1;

                    down.pairs[i].chip -= 1;
                    down.pairs[j].gen -= 1;
                    down.lift -= 1;

                    options.insert(up);
                    options.insert(down);
                }
                
                // (gen, chip)
                if self.pairs[i].gen == self.lift &&
                    self.pairs[j].chip == self.lift
                {
                    let mut up = self.clone();
                    let mut down = self.clone();

                    up.pairs[i].gen += 1;
                    up.pairs[j].chip += 1;
                    up.lift += 1;

                    down.pairs[i].gen -= 1;
                    down.pairs[j].chip -= 1;
                    down.lift -= 1;

                    options.insert(up);
                    options.insert(down);
                }
                
                // (gen, gen)
                if self.pairs[i].gen == self.lift &&
                    self.pairs[j].gen == self.lift &&
                    i != j
                {
                    let mut up = self.clone();
                    let mut down = self.clone();

                    up.pairs[i].gen += 1;
                    up.pairs[j].gen += 1;
                    up.lift += 1;

                    down.pairs[i].gen -= 1;
                    down.pairs[j].gen -= 1;
                    down.lift -= 1;

                    options.insert(up);
                    options.insert(down);
                }
            }
        }

        options.into_iter().filter(State::is_valid).collect()
    }
}



impl graph::Graph for State {
    fn adjacent(&self) -> Vec<Self> {
        self.next_states()
    }
}

fn main() {
    // Test case
    let start_part0 = State::new(
        vec![(0,1),(0,2)]
    );
    let end_part0 = State::with_floor(
        vec![(3, 3), (3, 3)],
        3
    );
    let now = SystemTime::now();
    let part0_ans = State::bfs(&start_part0, &end_part0);
    let part0_time = now.elapsed().unwrap();

    println!("[{: >3}s {: >6.2}ms] Part 0 number of moves = {}",
             part0_time.as_secs(),
             part0_time.subsec_nanos() as f64 * 1e-6_f64,
             part0_ans);

    // Part 1 case
    let start_part1 = State::new(
        vec![(0, 0), (2, 1), (2, 1), (2, 1), (2, 1)]
    );
    let end_part1 = State::with_floor(
        vec![(3, 3), (3, 3), (3, 3), (3, 3), (3, 3)],
        3
    );
    let now = SystemTime::now();
    let part1_ans = State::bfs(&start_part1, &end_part1);
    let part1_time = now.elapsed().unwrap();
    println!("[{: >3}s {: >6.2}ms] Part 1 number of moves = {}",
             part1_time.as_secs(),
             part1_time.subsec_nanos() as f64 * 1e-6_f64,
             part1_ans);
    
    let start_part2 = State::new(
        vec![(0, 0), (2, 1), (2, 1), (2, 1), (2, 1), (0, 0), (0, 0)]
    );
    let end_part2 = State::with_floor(
        vec![(3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 3)],
        3
    );
    let now = SystemTime::now();
    let part2_ans = State::bfs(&start_part2, &end_part2);
    let part2_time = now.elapsed().unwrap();
    println!("[{: >3}s {: >6.2}ms] Part 2 number of moves = {}",
             part2_time.as_secs(),
             part2_time.subsec_nanos() as f64 * 1e-6_f64,
             part2_ans);
}
