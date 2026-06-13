# syntax=docker/dockerfile:1

FROM rust:1.91.0-slim-bookworm AS builder
WORKDIR /usr/src/starboard

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# doing this allows caching of compiled dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY build.rs build.rs
COPY ./migrations ./migrations
RUN mkdir src
RUN echo "fn main() { dbg!(1); }" > src/main.rs
RUN cargo build --release
RUN rm -r src && rm target/release/starboard

# copy stuff over to the image that we need
COPY ./src ./src
COPY ./.sqlx ./.sqlx

# install starboard
RUN touch src/main.rs && cargo build --release

# get rid of cargo
FROM debian:bookworm-slim

# install certificates
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --shell /usr/sbin/nologin starboard

# copy starboard over from the builder
COPY --from=builder /usr/src/starboard/target/release/starboard /usr/local/bin/starboard

USER starboard

# run starboard
CMD ["starboard"]
