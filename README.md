# delog
A log-based filter tool that acts like a debugger.

This repository contains multiple Rust projects (crates) organized in a single workspace. The main components are:
- `delog`: The primary tool for monitoring logs and pausing execution based on specific keywords.
- `log_simulator`: A tool to simulate logs for testing purposes.

## Table of Contents
- [Overview](#overview)
- [Setup](#setup)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Overview

**Delog** is designed to help you debug applications by monitoring logs for specific keywords (e.g., `DEBUG`, `BREAKPOINT`) and pausing execution when such keywords are detected. This allows you to interactively decide when to continue execution, making debugging more efficient and effective.

## Setup

1. **Clone the repository:**

   ```sh
   git clone https://github.com/divanvisagie/delog.git
   cd delog_workspace
   ```

2. **Build the workspace:**

   ```sh
   cargo build
   ```

## Usage

1. **Run the log simulator:**

   The `log_simulator` generates logs that can be used to test `delog`.

   ```sh
   cargo run --bin log_simulator
   ```

2. **Use `delog` to monitor the log simulator:**

   Run `delog` with a command to monitor. For example, to monitor the log simulator:

   ```sh
   cargo run --bin delog -- cargo run --bin log_simulator
   ```

   When a log containing `BREAK` or `BREAKPOINT` is detected, `delog` will pause execution. Press `c` to continue.

## Project Structure

```
delog_workspace/
├── Cargo.toml
├── delog/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── log_simulator/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── delog_lib/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

- **delog/**: The main tool for monitoring logs.
- **log_simulator/**: A tool to simulate logs for testing purposes.
- **delog_lib/**: Shared library with common functionality.

## Development

1. **Add dependencies:**

   Add dependencies to the relevant `Cargo.toml` file.

2. **Build the project:**

   ```sh
   cargo build
   ```

3. **Run tests:**

   ```sh
   cargo test
   ```

4. **Run a specific binary:**

   ```sh
   cargo run --bin <binary-name>
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your changes. Ensure your code adheres to the existing style and includes appropriate tests.

## License

This project is licensed under the GPL v2 License. See the [LICENSE](LICENSE) file for details.


