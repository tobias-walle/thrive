run:
  just build-front
  cargo run -p thrive

dev:
  just dev-front & just dev-server

dev-front:
  cd frontend && yarn && yarn dev

dev-server:
  cargo watch -- cargo run -p thrive-server

check:
  cargo watch -- cargo check

fix:
  just fmt-front
  cargo fix --all-features --allow-staged --allow-dirty
  cargo fmt

build:
  just build-front
  cargo build --release

build-front:
  cd frontend && \
  yarn && \
  yarn build

fmt-front:
  cd frontend && \
  yarn fmt
 
# Check if dependencies are actually used
udeps:
  cargo +nightly udeps
   