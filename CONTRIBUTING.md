# Contributing to Garage UI

Thank you for your interest in contributing to Garage UI! This document provides guidelines and instructions for setting up the development environment and submitting contributions.

## Getting Started

Please refer to the "Quick Start" section in [README.md](README.md) for instructions on how to set up the project locally. You will need:
- Rust (1.92.0+)
- Node.js (24.11.0+)
- A running Garage instance

## Development Workflow

### Frontend (Angular)

The frontend is built with Angular 21, utilizing Signals and Angular Material for the UI.

- **Stack**: Angular 21, TypeScript, SCSS, Angular Material, Hono.js (BFF).
- **Directory**: `frontend/`
- **Linting & Formatting**: We use [Biome](https://biomejs.dev/) for linting and formatting.

### Backend (Rust)

The backend is a Rust application using Tokio and Tonic for gRPC communication with the frontend-web (via gRPC-Web).

- **Stack**: Rust, Tokio, Tonic, Axum/Hyper (implicit via Tonic/Tower).
- **Directory**: `./` (Root)
- **Linting & Formatting**: We use [rust-analyzer](https://rust-analyzer.github.io/) for linting and formatting.

### Protobuf & Code Generation

If you modify `.proto` files in the `proto/` directory, you must regenerate the code for both backend and frontend.

1.  **Backend**:
    ```bash
    cargo clean && cargo build
    ```
2.  **Frontend**:
    ```bash
    cd frontend
    npm run proto
    ```

## Pull Request Process

1.  **Fork** the repository and create your branch from `main`.
2.  **Create** a feature branch: `git checkout -b feat/my-new-feature`.
3.  **Commit** your changes following [Conventional Commits](https://www.conventionalcommits.org/):
    -   `feat: add new bucket dialog`
    -   `fix: resolve permission list loading issue`
    -   `docs: update readme`
    -   `style: format code`
    -   `refactor: simplify top-bar component`
4.  **Push** to your fork: `git push origin feat/my-new-feature`.
5.  **Submit** a Pull Request to the `main` branch.
    -   Provide a clear description of the changes.
    -   Link to any relevant issues.
    -   Ensure all checks pass (linting, build, tests).

## Code of Conduct

- [Frontend Architecture](frontend/ARCHITECTURE.md)
- [Backend Architecture](ARCHITECTURE.md)
