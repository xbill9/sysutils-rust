# Gemini Workspace for `sysutils-rust` (v0.2.0)

You are a Rust Developer working with Google Cloud.
You should follow Rust Best practices.
The recommended language level for Rust is 2024.

This document provides a developer-focused overview of the `sysutils-rust` project, tailored for use with Gemini.

## Project Overview

`sysutils-rust` is a Model Context Protocol (MCP) server written in Rust. It interacts via standard input/output (stdio) to provide system utility tools to MCP clients (like Claude Desktop or IDE extensions).

### Key Technologies

*   **Language:** [Rust](https://www.rust-lang.org/) (Edition 2024)
*   **MCP SDK:** [rmcp](https://crates.io/crates/rmcp) (v0.14.0) - Uses macros for tool definition and routing.
*   **System Info:** [sysinfo](https://crates.io/crates/sysinfo) (v0.37.x)
*   **Async Runtime:** [Tokio](https://tokio.rs/)
*   **Serialization:** [Serde](https://serde.rs/) & [Schemars](https://crates.io/crates/schemars) (for JSON-RPC and schema generation)
*   **Logging:** [Tracing](https://crates.io/crates/tracing) & [Tracing-Subscriber](https://crates.io/crates/tracing-subscriber) (JSON format to stderr)
*   **Containerization:** [Docker](https://www.docker.com/) (Distroless base)

## Architecture

*   **`src/main.rs`**: Single entry point. 
    *   `SysUtils` struct: Implements `ServerHandler` and `tool_router`.
    *   `get_system_info`: The primary MCP tool.
    *   `collect_system_info`: Shared logic for both MCP tool and CLI `info` command.
    *   `main`: Handles CLI arguments and initializes the MCP server on stdio.

## Getting Started

This project uses a `Makefile` to simplify common development tasks.

### Prerequisites

*   [Rust Toolchain](https://www.rust-lang.org/tools/install)
*   [Docker](https://docs.docker.com/get-docker/) (for container builds)
*   [Google Cloud SDK](https://cloud.google.com/sdk/docs/install) (for deployment)

### Environment Setup

Use the provided scripts for GCP and environment configuration:
- `source ./set_env.sh`: Sets up GCP project IDs, regions, and `RUST_LOG=trace`.
- `./set_adc.sh`: Authenticates with Google Application Default Credentials.

### Initial Build

1.  **Install Dependencies:**
    ```bash
    cargo build
    ```

2.  **Run the application locally (MCP mode):**
    ```bash
    make run
    ```
    *Note: The server runs on stdio and waits for JSON-RPC messages.*

3.  **Run CLI Info command:**
    ```bash
    cargo run -- info
    ```

## Development Workflow

### Code Quality

*   **Formatting:** `make fmt`
*   **Linting:** `make clippy`
*   **Checking:** `make check`

### Testing

```bash
make test
```
Current tests focus on schema generation verification.

## Deployment

Deployment configuration is managed via `cloudbuild.yaml`.

### Manual Deployment

To manually trigger a deployment to Google Cloud Build:

```bash
make deploy
```

This submits a build that creates a Docker image (`gcr.io`) and deploys it to Cloud Run.

### Dockerfile

The `Dockerfile` employs a multi-stage build:
1.  **Builder:** Compiles the Rust binary using `rust` image.
2.  **Runtime:** Copies the binary to a `gcr.io/distroless/cc-debian12` image.
*Note:* The Dockerfile currently exposes port 8080, though the app primarily uses stdio.

## Interacting with Gemini

You can use Gemini to help you with various tasks in this project. Relevant examples:

*   "Add a new tool to `SysUtils` in `main.rs` that checks disk usage for a specific path."
*   "Explain how `LazyLock` is used for `SYSTEM_INFO_SCHEMA`."
*   "Modify `collect_system_info` to include network interface information."
*   "Fix the Dockerfile to use the correct target for the release build."