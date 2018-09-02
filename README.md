# Intro

Simple tcp port scanner written in Rust.

NOTE: experimental toy project

# Usage

Scan all the tcp ports `0..65535`:

```
cargo run 127.0.0.1
```

Scan one tcp port:

```
cargo run mydomain.com 22
```

# Todo

[] add flags support
[] add concurrency
