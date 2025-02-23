# syntax=docker/dockerfile:1
# FROM public.ecr.aws/docker/library/rust:1.84.1 AS base
FROM rust:1.84 AS base
WORKDIR /seaography-mysql-example

FROM base AS dev
RUN <<EOF
rustup component add rustfmt clippy
cargo install bacon sea-orm-cli seaography-cli
EOF

FROM base AS builder
EXPOSE 3000
COPY . .
RUN cargo build --release

FROM busybox AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && update-ca-certificates
COPY --from=builder /timer/backend/target/release/web-app /usr/local/bin
COPY --from=builder /timer/backend/target/release/migration /usr/local/bin
EXPOSE 3000
CMD ["timer-backend"]
