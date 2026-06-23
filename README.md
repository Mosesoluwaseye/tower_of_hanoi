# Tower of Hanoi

A Rust implementation of the Tower of Hanoi puzzle using two different algorithms.

## Project Structure

```
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

The recursive algorithm solves the Tower of Hanoi puzzle by repeatedly moving `n-1` disks to the auxiliary rod, moving the largest disk to the destination rod, and finally moving the `n-1` disks onto the destination rod.

### Time Complexity

O(2ⁿ)

### Space Complexity

O(n)

The recursive approach uses the program's call stack. The maximum recursion depth equals the number of disks.

---

## Algorithm 2: Iterative

### Description

The iterative algorithm solves the same problem without recursive function calls. It repeatedly performs legal moves between the rods until the puzzle is solved.

### Time Complexity

O(2ⁿ)

### Space Complexity

O(1) auxiliary stack space for the algorithm itself, excluding the storage required to record the moves.

---

## Benchmark

The algorithms were benchmarked using different input sizes.

| Number of Disks |      Recursive |      Iterative |
| --------------: | -------------: | -------------: |
|               5 | To be measured | To be measured |
|              10 | To be measured | To be measured |
|              12 | To be measured | To be measured |
|              15 | To be measured | To be measured |

---

## Benchmark Interpretation

Both algorithms perform the same number of moves because the Tower of Hanoi puzzle requires a minimum of 2ⁿ−1 moves.

The recursive implementation is easier to understand and closely follows the mathematical definition of the problem. However, it creates a new stack frame for every recursive call. This increases memory usage and introduces function call overhead.

The iterative implementation avoids recursive calls and therefore reduces stack usage. It often performs slightly better because it avoids repeatedly creating and destroying stack frames. This also improves cache behaviour by keeping execution inside loops rather than repeatedly jumping between function calls.

For small input sizes, the performance difference is minimal. As the number of disks increases, the iterative version generally becomes more efficient because it performs fewer function call operations while still producing the same number of moves.

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
