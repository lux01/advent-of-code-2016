use std::fs::File;

fn main() {
    let mut ips = vec![true; std::u32::MAX as usize + 1];
    
    let input = {
        use std::io::Read;
        
        let mut strbuff = String::new();
        let mut f = File::open("input.txt").unwrap();
        f.read_to_string(&mut strbuff).unwrap();
        strbuff
    };

    for line in input.lines() {
        let parts = line.split('-').collect::<Vec<&str>>();
        let from = parts[0].parse::<usize>().unwrap();
        let to = parts[1].parse::<usize>().unwrap();

        for i in from .. (to + 1) {
            ips[i] = false;
        }
    }

    let mut first_found = false;
    let mut first = 0;
    let mut count = 0;

    for i in 0 .. ips.len() {
        if ips[i] {
            count += 1;

            if !first_found {
                first = i;
                first_found = true;
            }
        }
    }


    println!("First = {}", first);
    println!("Count = {}", count);
}
