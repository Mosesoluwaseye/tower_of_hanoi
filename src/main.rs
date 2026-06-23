use std::io;

use tower_of_hanoi::tower_of_hanoi;

fn main() {
    println!("Enter number of disks:");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().unwrap();

    let mut moves = Vec::new();

    tower_of_hanoi(n, 'A', 'C', 'B', &mut moves);

    println!("Total moves: {}", moves.len());

    for m in moves {
        println!("{}", m);
    }
}