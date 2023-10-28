# IDS 706 Mini Project 8 [![Tests](https://github.com/jaxonyue/Jaxon-Yue-Mini-Project-7/actions/workflows/tests.yml/badge.svg)](https://github.com/jaxonyue/Jaxon-Yue-Mini-Project-7/actions/workflows/tests.yml)

## Overview
* This repository includes the components for Mini-Project 8 - Rewrite a Python Script in Rust.

## Goal
* Rewrite an existing Python script in Rust for data processing
* Highlight improvements in speed and resource usage

## Key Elements in the Repo:
* project/src (contains the Rust script and main function)
* project/tests (contains the respective tests)
* project/Cargo.toml
* project/dataset (contains the csv file)
* mylib/lib.py (contains the Python version)
* Makefile
* Dockerfile
* devcontainer
* GitHub Actions
* format.sh
* lint.sh
* test.sh
* bashrc

## Data Processing Script
* For this project, I wrote a script in Rust to get the 25%, 50%, and 75% percentile of column of the "Annual Wages" data table. Then, I wrote the same function in Python and compared the performance between Rust and Python.


## Performance Comparison
* With the same data frame operation, Rust has beat the Python version during all my tries, both in run time and CPU and memory usage.

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



## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
