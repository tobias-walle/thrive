run:
  just build-front
  cargo run -p thrive

dev:
  just dev-front & just dev-server

dev-front:
  cd frontend && yarn && yarn dev

dev-server:
  cargo watch -- cargo run -p thrive-server

clippy:
  cargo watch -- cargo clippy

fix:
  just fmt-front
  just fix-server

fix-server:
  cargo fix --all-features --allow-staged --allow-dirty
  cargo +nightly clippy --fix -Z unstable-options --allow-staged --allow-dirty
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
   
generate-schemas:
  cargo run --package thrive-core --bin generate-json-schemas