# Stage 1: Build Rust Backend
FROM rust:1.93-slim-bookworm AS backend-builder

WORKDIR /usr/src/app

# Install dependencies for building (protobuf, ssl)
RUN apt-get update && \
    apt-get install -y protobuf-compiler libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies first (caching)
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release

# Remove the dummy build
RUN rm -f target/release/deps/garage_ui*

# Copy source code and proto
COPY src ./src
COPY proto ./proto

# Build the application
RUN cargo build --release

# Stage 2: Build Angular Frontend + BFF
FROM node:24-bookworm-slim AS frontend-builder

WORKDIR /app

# Install dependencies
COPY frontend/package*.json ./
RUN npm ci

# Copy source code
COPY frontend/ .

# Build Angular application (Output: dist/garage-ui/browser)
RUN npm run build

# Build Server (TypeScript to JavaScript) (Output: out-tsc/server)
RUN npx tsc -p tsconfig.server.json
RUN npx tsc-alias -p tsconfig.server.json

# Remove devDependencies
RUN npm prune --omit=dev

# Stage 3: Runtime (Unified)
FROM node:24-bookworm-slim

WORKDIR /app

# Install runtime dependencies for Rust binary and general tools
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# --- Setup Frontend ---
# Copy built artifacts from frontend-builder
COPY --from=frontend-builder /app/dist ./dist
COPY --from=frontend-builder /app/out-tsc/server ./dist-server
COPY --from=frontend-builder /app/package*.json ./
COPY --from=frontend-builder /app/node_modules ./node_modules

# --- Setup Backend ---
# Copy binary from backend-builder
COPY --from=backend-builder /usr/src/app/target/release/garage-ui /app/garage-ui

# Copy entrypoint script
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

# Expose ports
# 3000: Frontend/BFF
# 50051: Backend gRPC (Internal mostly, but exposed for debug)
EXPOSE 3000 50051

# Environment variables
ENV PORT=3000
ENV GRPC_URI=http://localhost:50051

# Start both services
CMD ["./start.sh"]
