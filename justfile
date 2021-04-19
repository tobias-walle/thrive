fix:
  just fmt-front
  cargo fix --all-features
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
  