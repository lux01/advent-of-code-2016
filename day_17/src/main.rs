extern crate crypto;

mod graph;

use graph::Graph;
use crypto::digest::Digest;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Path {
    x: u8,
    y: u8,
    history: String
}

impl Path {
    pub fn new(hash: &str) -> Path {
        Path { x: 0, y: 0, history: hash.to_owned() }
    }
}

fn is_open(c: char) -> bool {
    c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f'
}

impl Graph for Path {
    fn adjacent(&self) -> Vec<Self> {
        let mut opts = Vec::new();
        let mut md5 = crypto::md5::Md5::new();

        md5.input_str(&self.history);
        let hash = md5.result_str().chars().take(4).collect::<Vec<char>>();

        if is_open(hash[0]) && self.y > 0 {
            let mut up = self.clone();
            up.y -= 1;
            up.history.push('U');
            opts.push(up);
        }

        if is_open(hash[1]) && self.y < 3 {
            let mut down = self.clone();
            down.y += 1;
            down.history.push('D');
            opts.push(down);
        }

        if is_open(hash[3]) && self.x < 3 {
            let mut right = self.clone();
            right.x += 1;
            right.history.push('R');
            opts.push(right);
        }

        if is_open(hash[2]) && self.x > 0 {
            let mut left = self.clone();
            left.x -= 1;
            left.history.push('L');
            opts.push(left);
        }
        
        opts
    }
}

fn main() {
    let test_0 = "ihgpwlah";
    let test_1 = "kglvqrro";
    let test_2 = "ulqzkmiv";
    let challenge = "dmypynyp";
    
    println!("Test 0 = {}",
             Path::bfs(&Path::new(test_0), |ref p| p.x == 3 && p.y == 3).history
             .split(test_0).skip(1).next().unwrap()
    );

    println!("Test 1 = {}",
             Path::bfs(&Path::new(test_1), |ref p| p.x == 3 && p.y == 3).history
             .split(test_1).skip(1).next().unwrap()
    );

    println!("Test 2 = {}",
             Path::bfs(&Path::new(test_2), |ref p| p.x == 3 && p.y == 3).history
             .split(test_2).skip(1).next().unwrap()
    );

    println!("Challenge = {}",
             Path::bfs(&Path::new(challenge), |ref p| p.x == 3 && p.y == 3).history
             .split(challenge).skip(1).next().unwrap()
    );

    println!("Maximum number of steps = {:#?}",
             Path::bfs_ends(&Path::new(challenge), |ref p| p.x == 3 && p.y == 3)
             .into_iter()
             .map(|p| p.history.split(challenge).skip(1).next().unwrap().len())
             .max().unwrap());
}
