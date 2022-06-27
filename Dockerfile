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

FROM gcr.io/distroless/cc@sha256:bac217056540e5330875164e5d6e29b2fcd2725ed0994332b6a8650d57ddd94d
COPY --from=builder /build/target/release/rustreleaser /
ENTRYPOINT ["/rustreleaser"]
