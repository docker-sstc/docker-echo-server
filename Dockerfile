FROM --platform=linux/amd64 messense/rust-musl-cross:x86_64-musl as builder-amd64

# We need to add the source code to the image because `rust-musl-builder`
# assumes a UID of 1000, but TravisCI has switched to 2000.
ADD . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM --platform=linux/amd64 scratch
COPY --from=builder-amd64 /home/rust/src/target/x86_64-unknown-linux-musl/release/echo-server /app/main
WORKDIR /app
CMD ["./main"]

FROM --platform=linux/arm64 messense/rust-musl-cross:aarch64-musl as builder-arm64

ADD . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM --platform=linux/arm64 scratch
COPY --from=builder-arm64 /home/rust/src/target/aarch64-unknown-linux-musl/release/echo-server /app/main
WORKDIR /app
CMD ["./main"]
