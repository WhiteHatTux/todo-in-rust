# https://github.com/docker/buildx/issues/395#issuecomment-749769678
FROM --platform=$BUILDPLATFORM rust:1.52.0 as fetcher

ENV USER=root
WORKDIR /usr/app/workdir/
RUN cargo init
COPY Cargo.toml /usr/app/workdir/Cargo.toml
COPY Cargo.lock /usr/app/workdir/Cargo.lock
RUN mkdir -p /usr/app/workdir/.cargo && cargo vendor > /usr/app/workdir/.cargo/config

FROM rust:1.52.0 as build

WORKDIR /usr/app/workdir
# start cargo workaround for docker layer cache
RUN mkdir src
RUN echo "fn main() {}" > src/dummy.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN echo '\n[[bin]]\nname = "download-only"\npath = "src/dummy.rs"\n' >> Cargo.toml
COPY --from=fetcher /usr/app/workdir/.cargo /usr/app/workdir/.cargo
COPY --from=fetcher /usr/app/workdir/vendor /usr/app/workdir/vendor
RUN cargo build --release
RUN rm Cargo.toml src/dummy.rs
# end cargo workaround for docker layer cache

COPY . .
RUN ls target/release/deps
RUN cargo build --release --frozen --locked


FROM debian:buster-slim

COPY --from=build /usr/app/workdir/target/release/todo-in-rust-with-tide /app/todo-in-rust
CMD ["/app/todo-in-rust"]
