#!/bin/sh

# Start Rust Backend in background
echo "Starting Backend..."
./garage-ui &
BACKEND_PID=$!

# Wait for Backend to be ready (Optional: simple sleep or health check)
# Implementing a simple wait is good practice, but for now we just start both.
echo "Backend started with PID $BACKEND_PID"

# Start Node Frontend in foreground
echo "Starting Frontend..."
# Exec node so it receives signals
# But we need to cleanup backend on exit
# So we run node as a child and wait

node dist-server/server/main.js &
FRONTEND_PID=$!
echo "Frontend started with PID $FRONTEND_PID"

# Helper function to kill processes
cleanup() {
    echo "Stopping processes..."
    kill $BACKEND_PID
    kill $FRONTEND_PID
    exit 0
}

# Trap SIGTERM and SIGINT
trap cleanup SIGTERM SIGINT

# Wait for processes
wait $FRONTEND_PID
wait $BACKEND_PID
