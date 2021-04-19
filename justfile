fix:
  cargo fix --all-features
  cargo fmt

build:
  just build-front
  just build-back

build-front:
  cd frontend && \
  yarn && \
  yarn build

build-back:
  cargo build --release