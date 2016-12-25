pub trait Scrambler {
    fn new(password: &str) -> Self;
    fn get_password(&self) -> String;

    fn find_el(&self, x: char) -> usize;
    
    fn swap_pos(&mut self, x: usize, y: usize);
    fn swap_lett(&mut self, x: char, y: char) {
        let x_idx = self.find_el(x);
        let y_idx = self.find_el(y);
        self.swap_pos(x_idx, y_idx);
    }
    
    fn rotate_left(&mut self, n: usize);
    fn rotate_right(&mut self, n: usize);
    fn rotate_letter(&mut self, x: char);
    fn reverse_through(&mut self, x: usize, y: usize);
    fn move_pos(&mut self, x: usize, y: usize);
}
