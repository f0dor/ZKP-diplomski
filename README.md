# Zero-Knowledge Proof (ZKP) Project

This repository contains the implementation of a Zero-Knowledge Proof (ZKP) using the Bulletproofs library in Rust. The goal of this project is to prove that a number lies within the range [18, 100] without revealing the actual number. This project is part of my final thesis.

## Features
- Implements range proofs using the Bulletproofs library.
- Demonstrates the use of ZKP for privacy-preserving computations.

## Prerequisites
- Rust programming language installed on your system.

## Installation

### Step 1: Install Rust
1. Download and install Rust by running the following command in your terminal:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. Follow the on-screen instructions to complete the installation.
3. After installation, ensure Rust is properly installed by running:
    ```bash
    rustc --version
    ```
    This should display the installed Rust version.

### Step 2: Clone the Repository
Clone this repository to your local machine:
```bash
git clone <repository-url>
cd <repository-folder>
```

## Running the Code
1. Navigate to the project directory:
    ```bash
    cd <repository-folder>
    ```
2. Build the project:
    ```bash
    cargo build
    ```
3. Run the code:
    ```bash
    cargo run
    ```

## Project Structure
- `src/`: Contains the Rust source code.
- `Cargo.toml`: Configuration file for the Rust project.

## References
- [Bulletproofs Library](https://github.com/dalek-cryptography/bulletproofs)
- [Rust Programming Language](https://www.rust-lang.org/)

Feel free to explore the code and reach out if you have any questions!