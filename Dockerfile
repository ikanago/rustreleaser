FROM lukemathwalker/cargo-chef:0.1.35-rust-1.59.0-bullseye AS chef

FROM chef AS planner
WORKDIR /plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /build
COPY --from=planner /plan/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc@sha256:1b82fde9abdd6b83077fa99af6b7bb93fcde1e93325eb00bfb814d5068ce60d9
COPY --from=builder /build/target/release/rustreleaser /
ENTRYPOINT ["/rustreleaser"]
