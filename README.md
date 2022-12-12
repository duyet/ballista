# Ballista: Distributed Scheduler

[Ballista](https://github.com/apache/arrow-ballista) is a distributed compute platform primarily implemented in Rust.

## Starting a cluster

Local cluster for testing purposes

```bash
cargo install --locked ballista-scheduler
cargo install --locked ballista-executor
```

With these crates installed, it is now possible to start a scheduler process.

```bash
RUST_LOG=info ballista-scheduler
```

Next, start an executor processes in a new terminal session with the specified concurrency level.

```bash
RUST_LOG=info ballista-executor --bind-port 50051 -c 4
RUST_LOG=info ballista-executor --bind-port 50052 -c 4
RUST_LOG=info ballista-executor --bind-port 50052 -c 4
```

## Executing a query

```bash
cargo run
```
