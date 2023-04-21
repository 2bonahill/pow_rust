# PoW Rust
Rust implementation of a basic Proof of Work (PoW) algorithm.

## Usage
```rust
// Generate some random input data
let in_hash: InHash = [0; IN_SIZE].map(|_| -> u8 { rand::thread_rng().gen() });

for i in 0..10 {
    let start = Instant::now();
    let x = generate_pow(&in_hash, 1 << i);
    let duration = start.elapsed();
    println!(
        "Round number: {} - {} // Time: {}ns",
        i,
        &x,
        duration.as_nanos()
    );
}
```

The expected output is:
```shell
Round number: 0 - 085e336e38d076fd // Time: 60055ns
Round number: 1 - 60525c022dfd815e // Time: 33187ns
Round number: 2 - cefc5075c0575549 // Time: 27849ns
Round number: 3 - 8234113df33af383 // Time: 280743ns
Round number: 4 - 9e2e2657af1ddd18 // Time: 385199ns
Round number: 5 - 0797fb538a6306ba // Time: 142852ns
Round number: 6 - 389cdc37da69261f // Time: 1561866ns
Round number: 7 - d2765b378cb4adb9 // Time: 2113137ns
Round number: 8 - 33f1a16677f7a496 // Time: 13373398ns
Round number: 9 - 6aea4bb553d5b8b5 // Time: 13912648ns
```