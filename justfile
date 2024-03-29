# List commands
[private]
list:
  just --list

# Start the development server
dev:
  cargo tauri dev

# Check for errors and warnings
check:
  cargo clippy --all

# Check for errors and warnings and fix them if possible
fix:
  cargo clippy --all --fix --allow-staged
  cargo fmt

# Run the tests of all workspaces
test:
  cargo test --all

# Run the tests of all workspaces
test-watch:
  cargo watch -- just test
