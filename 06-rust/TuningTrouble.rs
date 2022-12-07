use std::env::args;
use std::fs::File;
use std::io::Read;

fn read_buffer(filename: String) -> String {
    let mut file = File::open(filename).expect("Could not open file");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Could not read file");
    string
}

fn find_msg_part(buffer: &String, n_unique: usize) -> usize {
    buffer
        .as_bytes()
        .windows(n_unique)
        .enumerate()
        .filter(|(_, win)| {
            let mut win_copy = win.to_vec();
            win_copy.sort();
            win_copy.windows(2).all(|w| w[0] != w[1])
        })
        .map(|(i, _)| i + n_unique)
        .next()
        .expect("No start sequence found")
}

fn main() {
    let filename = args().skip(1).next().unwrap();
    let buffer = read_buffer(filename);

    println!("Part 1: {}", find_msg_part(&buffer, 4));
    println!("Part 2: {}", find_msg_part(&buffer, 14));
}
