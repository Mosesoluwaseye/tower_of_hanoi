pub fn tower_of_hanoi(
    n: u32,
    from: char,
    to: char,
    aux: char,
    moves: &mut Vec<String>,
) {
    if n == 0 {
        return;
    }

    tower_of_hanoi(n - 1, from, aux, to, moves);

    moves.push(format!("Move disk {} from {} to {}", n, from, to));

    tower_of_hanoi(n - 1, aux, to, from, moves);
}
pub fn tower_of_hanoi_iterative(
    n: u32,
    from: char,
    to: char,
    aux: char,
) -> Vec<String> {
    let mut moves = Vec::new();

    tower_of_hanoi(n, from, to, aux, &mut moves);

    moves
}