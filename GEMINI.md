# Gemini Workspace for `sysutils-rust`

You are a Rust Developer working with Google Cloud.
You should follow Rust Best practices.
The recommended language level for Rust is 2024.

This document provides a developer-focused overview of the `sysutils-rust` project, tailored for use with Gemini.

## Project Overview

`sysutils-rust` is a Model Context Protocol (MCP) server written in Rust. It interacts via standard input/output (stdio) to provide system utility tools to MCP clients (like Claude Desktop or IDE extensions).

### Key Technologies

*   **Language:** [Rust](https://www.rust-lang.org/) (Edition 2024)
*   **MCP SDK:** [rmcp](https://crates.io/crates/rmcp) (Rust Model Context Protocol)
*   **System Info:** [sysinfo](https://crates.io/crates/sysinfo)
*   **Async Runtime:** [Tokio](https://tokio.rs/)
*   **Serialization:** [Serde](https://serde.rs/) & [Schemars](https://crates.io/crates/schemars)
*   **Containerization:** [Docker](https://www.docker.com/)

## Getting Started

This project uses a `Makefile` to simplify common development tasks.

### Prerequisites

*   [Rust Toolchain](https://www.rust-lang.org/tools/install)
*   [Docker](https://docs.docker.com/get-docker/) (for container builds)
*   [Google Cloud SDK](https://cloud.google.com/sdk/docs/install) (for deployment)

### Initial Setup

1.  **Install Dependencies:**
    ```bash
    cargo build
    ```

2.  **Run the application locally:**
    ```bash
    make run
    ```
    *Note: The server runs on stdio and waits for JSON-RPC messages. It does not bind to a network port by default.*

## Development Workflow

The `Makefile` provides targets for common development tasks.

### Building the Project

*   **Development Build:**
    ```bash
    make build
    ```
*   **Release Build:**
    ```bash
    make release
    ```

### Code Quality

*   **Formatting:**
    ```bash
    make fmt
    ```
*   **Linting:**
    ```bash
    make clippy
    ```
    *Ensure `ktlint` equivalent for Rust (rustfmt/clippy) is passed.*

### Testing

```bash
make test
```

## Deployment

Deployment configuration is managed via `cloudbuild.yaml`.

### Manual Deployment

To manually trigger a deployment to Google Cloud Build:

```bash
make deploy
```

This submits a build that creates a Docker image and pushes it to the Container Registry.

### Dockerfile

The `Dockerfile` employs a multi-stage build:
1.  **Builder:** Compiles the Rust binary using a standard Rust image.
2.  **Runtime:** Copies the binary to a `distroless` image for a minimal and secure footprint.

## Interacting with Gemini

You can use Gemini to help you with various tasks in this project. Here are relevant examples:

*   "Add a new tool to `main.rs` that checks disk usage for a specific path."
*   "Explain how the `rmcp` macros work in `SysUtils` struct."
*   "Write a test for the `get_system_info` function."
*   "Optimize the Dockerfile for smaller image size."