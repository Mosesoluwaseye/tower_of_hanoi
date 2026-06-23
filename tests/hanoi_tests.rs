use tower_of_hanoi::tower_of_hanoi;

#[test]
fn test_three_disks() {
    let mut moves = Vec::new();

    tower_of_hanoi(3, 'A', 'C', 'B', &mut moves);

    assert_eq!(moves.len(), 7);
}

#[test]
fn test_one_disk() {
    let mut moves = Vec::new();

    tower_of_hanoi(1, 'A', 'C', 'B', &mut moves);

    assert_eq!(moves.len(), 1);
}