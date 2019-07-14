# Project Circles: Coordinator
The coordinator is a centralized web application that helps connecting circle nodes to each other and provides a public interface to interact and connect with circle nodes.

## Setup
The coordinator is written in Rust. To run an coordinator server, first download [Rust](https://www.rust-lang.org/tools/install). Then clone this repository. Within this repository, set the toolchain to use the latest version of Rust nightly using the following command:

```
rustup override set nightly
```

The coordinator server utilizes the [Rocket](https://rocket.rs/) web framework which requires the latest nightly toolchain. Now you should be able to start a server locally by running `cargo run`. This command should spin up a web server with an interface in your local [environment](http://localhost:8000).
