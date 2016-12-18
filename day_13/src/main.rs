mod graph;
use graph::Graph;

pub type Position = (usize, usize, usize);

use std::time::SystemTime;

fn is_open(p: &Position) -> bool {
    let mut val = p.0*p.0 + 3*p.0 + 2*p.0*p.1 + p.1 + p.1*p.1 + p.2;
    
    if val == 0 {
        return true;
    }
    
    let mut num_1s = 0;
    
    while val != 0 {
        num_1s += val % 2;
        val = val >> 1;
    }
    
    num_1s % 2 == 0
}

impl Graph for Position {
    fn adjacent(&self) -> Vec<Position> {
        let mut opts = Vec::new();
        for &(i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            if (i < 0 && self.0 == 0) || (j < 0 && self.1 == 0) {
                continue;
            }
            let pos = ((self.0 as isize + i) as usize,
                       (self.1 as isize + j) as usize,
                       self.2);
            if is_open(&pos) {
                opts.push(pos);
            }
        }
        opts
    }
}

fn main() {
        // Test case
    let start_part0 = (1, 1, 10);
    let end_part0 = (7, 4, 10);
    let now = SystemTime::now();
    let part0_ans = Position::bfs(&start_part0, &end_part0, true);
    let part0_time = now.elapsed().unwrap();

    println!("[{: >3}s {: >6.2}ms] Part 0 number of moves = {}",
             part0_time.as_secs(),
             part0_time.subsec_nanos() as f64 * 1e-6_f64,
             part0_ans);
    
    // Part 1
    let start_part1 = (1, 1, 1364);
    let end_part1 = (31, 39, 1364);
    let now = SystemTime::now();
    let part1_ans = Position::bfs(&start_part1, &end_part1, true);
    let part1_time = now.elapsed().unwrap();

    println!("[{: >3}s {: >6.2}ms] Part 1 number of moves = {}",
             part1_time.as_secs(),
             part1_time.subsec_nanos() as f64 * 1e-6_f64,
             part1_ans);

    // The furthest one can move in 50 moves is 50 units in the x direction
    // and 50 units in the y direction. 
    let mut posns = Vec::new();
    for x in 0..51 {
        for y in 0..51 {
            if is_open(&(x, y, 1364)) {
                posns.push((x, y, 1364));
            }
        }
    }

    let now = SystemTime::now();
    let part2_ans = posns.into_iter()
        .map(|p| Position::bfs(&start_part1, &p, true))
        .filter(|&d| d <= 50)
        .count();
    let part2_time = now.elapsed().unwrap();
    println!("[{: >3}s {: >6.2}ms] Part 2 number of spaces = {}",
             part2_time.as_secs(),
             part2_time.subsec_nanos() as f64 * 1e-6_f64,
             part2_ans);

}
