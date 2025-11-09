FROM rust:1.91 AS base
WORKDIR /app

FROM base AS development
RUN rustup component add rustfmt clippy
CMD ["bash"]

FROM base AS builder
COPY . .
RUN cargo build --release

FROM base AS runtime
RUN <<EOT
apt-get update
apt-get install -y --no-install-recommends ca-certificates libasound2-dev libdbus-1-dev
update-ca-certificates
rm -rf /var/lib/apt/lists/*
EOT
COPY --from=builder /app/target/release/pomodoro /usr/local/bin/pomodoro
RUN <<EOT
useradd -r -s /bin/false appuser
chown appuser:appuser /usr/local/bin/pomodoro
EOT
USER appuser
CMD ["pomodoro"]
