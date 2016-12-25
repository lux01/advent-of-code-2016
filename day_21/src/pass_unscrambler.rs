use super::scrambler::Scrambler;


#[derive(Debug, PartialEq, Eq)]
pub struct PassUnscrambler {
    state: Vec<char>,
}

impl Scrambler for PassUnscrambler {
    fn new(password: &str) -> Self {
        PassUnscrambler {
            state: password.chars().collect(),
        }
    }

    fn get_password(&self) -> String {
        self.state.clone().into_iter().collect()
    }


    fn find_el(&self, x: char) -> usize {
        self.state.iter()
            .enumerate()
            .filter(|&(_, c)| *c == x)
            .map(|(i, _)| i)
            .next()
            .unwrap()
    }
    
    fn swap_pos(&mut self, x: usize, y: usize) {
        self.state.swap(x, y);
    }

    fn rotate_left(&mut self, n: usize) {
        let mut n = n;
        while n > 0 {
            let back = self.state.pop().unwrap();
            self.state.insert(0, back);
            n -= 1;
        }
    }

    fn rotate_right(&mut self, n: usize) {
        let mut n = n;
        while n > 0 {
            let top = self.state.remove(0);
            self.state.push(top);
            n -= 1;
        }
    }

    fn rotate_letter(&mut self, x: char) {
        let idx = self.find_el(x);
        let mut lookup = (0 .. self.state.len())
            .map(|i| (i, if i >= 4 { 2*i + 2 } else { 2*i + 1 } % self.state.len()))
            .collect::<Vec<_>>();
        lookup.sort_by(|&a, &b| a.1.cmp(&b.1));
        let lookup = lookup.into_iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
      
        let i = lookup[idx];

        self.rotate_right(if i >= 4 { i + 2 } else { i + 1 });
    }

    fn reverse_through(&mut self, x: usize, y: usize) {
        let mut middle = self.state.split_off(x);
        let rest = middle.split_off(y - x + 1);
        middle.reverse();

        self.state = self.state.clone().into_iter()
            .chain(middle.into_iter())
            .chain(rest.into_iter())
            .collect();
    }

    fn move_pos(&mut self, x: usize, y: usize) {
        let y_el = self.state.remove(y);
        self.state.insert(x, y_el);
    }

}
