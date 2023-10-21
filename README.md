# IDS 706 Mini Project 7

## Overview
* This repository includes the components for Mini-Project 7 - Package a Rust Script into a Command-Line Tool.

## Goal
* Package a Rust script with setuptools or a similar tool
* Include a user guide on how to install and use the tool

## Key Elements in the Repo:
* caesar-cipher-cli/src (contains the Rust script and main function)
* caesar-cipher-cli/tests (contains the respective tests)
* caesar-cipher-cli/Cargo.toml
* Makefile
* Dockerfile
* devcontainer
* GitHub Actions
* format.sh
* lint.sh
* test.sh
* bashrc

## User Guide to Run
1. Fork the repository at **https://github.com/nogibjj/rust-data-engineering**
2. Install Rust by running
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Restart shell by running
```
source $HOME/.cargo/env
```
4. Check to see if Rust has been installed correctly
```
rustc --version
cargo --version
```
5. Now, you can run `cargo build` to compile your changes  
```
cargo build
```
6.  Run `cargo run` to test your modified tool 
```
cargo run
```

## Rust Script
You can do three things using my tool: **1) encrypt a string, 2) decrypt a string, and 3) make a string all caps**
**To encrypt a string**
```
cargo run --  --message "your string" --encrypt --shift "your shift value"
```
**To decrypt a string**
```
cargo run --  --message "your string" --decrypt --shift "your shift value"
```
Here's an example of encrypting and decrypting the same message:
<img width="916" alt="Screenshot 2023-10-21 at 6 02 14 PM" src="https://github.com/jaxonyue/Jaxon-Yue-Mini-Project-7/assets/70416390/1e3b6359-4449-42c1-abe6-26dd26124f09">
**To make a string all caps**
```
cargo run --  --message "your string" --caps
```
Here's an example of making a string all caps:
<img width="895" alt="Screenshot 2023-10-21 at 6 02 22 PM" src="https://github.com/jaxonyue/Jaxon-Yue-Mini-Project-7/assets/70416390/30fca7f5-f197-4c30-80b1-471d35756b6b">

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```

I have passed all GitHub Actions as below:
<img width="787" alt="Screenshot 2023-10-21 at 6 04 43 PM" src="https://github.com/jaxonyue/Jaxon-Yue-Mini-Project-7/assets/70416390/8493a7a1-005a-4535-a532-b5e534cb2f22">


## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
