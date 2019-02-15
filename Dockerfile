FROM ekidd/rust-musl-builder as builder

# We need to add the source code to the image because `rust-musl-builder`
# assumes a UID of 1000, but TravisCI has switched to 2000.
ADD . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/echo-server /app/main
WORKDIR /app
CMD ["./main"]
