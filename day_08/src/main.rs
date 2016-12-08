use std::fs::File;
use std::io::Read;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn rect(pixels: &mut [[bool; WIDTH]; HEIGHT], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            pixels[y][x] = true;
        }
    }
}

fn rotate_row(pixels: &mut [[bool; WIDTH]; HEIGHT], y: usize, amount: usize) {
    let old = pixels.clone();
    for x in 0..WIDTH {
        pixels[y][x] = old[y][(x + WIDTH - amount) % WIDTH];
    }
}

fn rotate_col(pixels: &mut [[bool; WIDTH]; HEIGHT], x: usize, amount: usize) {
    let old = pixels.clone();
    for y in 0..HEIGHT {
        pixels[y][x] = old[(y + HEIGHT - amount) % HEIGHT][x];
    }
}

fn number_lit(pixels: &[[bool; WIDTH]; HEIGHT]) -> usize {
        pixels.iter().flat_map(|r| r.iter()).filter(|&&p| p).count()
}

fn print_screen(pixels: &[[bool; WIDTH]; HEIGHT]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if pixels[y][x] { '#' } else { ' ' });
        }
        print!("\n");
    }
}

macro_rules! get_nums {
    ($line:expr, $split:expr) => {{
        $line.split($split)
            .map(|s| s.chars()
                 .filter(|&c| c.is_numeric())
                 .collect::<String>()
                 .parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }}
}

fn main() {
    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();

    let mut screen = [[false; WIDTH]; HEIGHT];

    for line in input.lines() {
        if line.starts_with("rect") {
            let nums = get_nums!(line, 'x');
            rect(&mut screen, nums[0], nums[1]);
        } else if line.starts_with("rotate row") {
            let nums = get_nums!(line, "by");
            rotate_row(&mut screen, nums[0], nums[1]);
        } else if line.starts_with("rotate column") {
            let nums = get_nums!(line, "by");
            rotate_col(&mut screen, nums[0], nums[1]);
        }
    }
    print_screen(&screen);
    println!("Number of lit pixels = {}", number_lit(&screen));
}
