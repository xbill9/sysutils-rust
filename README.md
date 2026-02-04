# sysutils-rust (v0.2.0)

A Rust-based [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server that provides system utility tools. This application runs over standard input/output (stdio) and allows MCP clients to access detailed system information.

## Overview

`sysutils-rust` implements an MCP server using the `rmcp` library. It exposes tools to retrieve system metrics such as kernel version, CPU usage, memory statistics, and disk information.

## Features

*   **MCP Protocol Support**: Implements the Model Context Protocol over stdio.
*   **System Information**: Provides a detailed report of the host system.
    *   **Tool**: `get_system_info`
    *   **Collected Data**: 
        *   **System**: Name, Kernel version, OS version, Host name.
        *   **CPU**: Number of cores.
        *   **Memory**: Total/Used RAM and Total/Used Swap (in MB).
        *   **Disk**: Name, File system, Total/Available space (in GB) for all mounted disks.
*   **Logging**: Structured JSON logging to `stderr` using `tracing-subscriber`.
*   **CLI Mode**: Direct execution to print system information without starting the MCP server.

## Getting Started

### Prerequisites

*   [Rust Toolchain](https://www.rust-lang.org/tools/install) (Edition 2024)
*   [Make](https://www.gnu.org/software/make/) (optional, for convenience)

### Build

You can build the project using Cargo or the provided Makefile.

**Using Cargo:**
```bash
cargo build --release
```

**Using Makefile:**
```bash
make release
```

The executable will be located at `target/release/sysutils-rust`.

## Usage

### As an MCP Server

This application is designed to be run by an MCP client (such as Claude Desktop or an MCP-compliant IDE extension).

**Example Configuration (Claude Desktop):**

Add the following to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "sysutils": {
      "command": "/absolute/path/to/sysutils-rust/target/release/sysutils-rust",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Running Locally (MCP Mode)

To run the server manually (waiting for JSON-RPC messages on stdin):

```bash
make run
# or
./target/release/sysutils-rust
```

Note: The server will start and wait for input. You won't see a prompt. It communicates via JSON-RPC messages. Structured logs will appear on `stderr`.

### CLI Usage (Direct Info)

You can also run the system information tool directly from the command line:

```bash
cargo run -- info
# or
./target/release/sysutils-rust info
```
This will print the system information report to `stdout` and exit.

## Development

The project includes a `Makefile` to streamline development tasks.

*   **Format Code**: `make fmt`
*   **Lint Code**: `make clippy`
*   **Run Tests**: `make test`
*   **Clean Build**: `make clean`
*   **Check Code**: `make check`

## Deployment

### Docker & Google Cloud Run

The project is containerized and ready for deployment to Google Cloud Run. 

*   **Build Docker Image**: `make docker-build`
*   **Deploy to Cloud Build**: `make deploy` (Requires Google Cloud SDK)

*Note: While a Dockerfile and Cloud Build configuration are provided, MCP servers over stdio are typically used in local environments or as sidecars.*