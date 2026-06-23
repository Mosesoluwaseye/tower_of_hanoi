# Tower of Hanoi

A Rust implementation of the Tower of Hanoi puzzle using two different algorithms.

## Project Structure

```text
tower_of_hanoi/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── hanoi_tests.rs
├── benches/
│   └── benchmark.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Algorithm 1: Recursive

### Description

The recursive algorithm solves the Tower of Hanoi puzzle by recursively moving `n - 1` disks to the auxiliary rod, moving the largest disk to the destination rod, and then moving the `n - 1` disks onto the destination rod.

### Time Complexity

**O(2ⁿ)**

### Space Complexity

**O(n)**

The recursive implementation uses the call stack. The recursion depth increases linearly with the number of disks.

---

## Algorithm 2: Iterative

### Description

The iterative algorithm solves the same problem without recursive function calls by repeatedly performing legal moves between the rods until the puzzle is solved.

### Time Complexity

**O(2ⁿ)**

### Space Complexity

**O(1)** auxiliary stack space, excluding the storage required for recording moves.

---

## Benchmark

The benchmark was performed using the Criterion benchmarking library.

| Algorithm            | Average Execution Time |
| -------------------- | ---------------------: |
| Recursive (10 disks) |              448.18 µs |
| Iterative (10 disks) |              337.40 µs |

---

## Benchmark Interpretation

Both algorithms perform the minimum number of moves required to solve the Tower of Hanoi puzzle, which is 2ⁿ−1.

The recursive implementation is simple and closely follows the mathematical definition of the problem. However, each recursive call creates a new stack frame, increasing memory usage and adding function call overhead.

The iterative implementation avoids recursive calls and therefore reduces stack usage. In this benchmark it completed faster than the recursive implementation.

Although both algorithms have the same asymptotic time complexity of **O(2ⁿ)**, the iterative implementation performed better because it eliminates the overhead associated with recursive function calls.

---

## Running the Project

```bash
cargo run
```

---

## Running the Tests

```bash
cargo test
```

---

## Running the Benchmark

```bash
cargo bench
```

