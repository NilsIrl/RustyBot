FROM ekidd/rust-musl-builder:nightly-2019-11-06 as builder

COPY ./ ./
RUN cargo build --release
RUN strip ./target/x86_64-unknown-linux-musl/release/rusty_bot

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rusty_bot ./rusty_bot
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

CMD ["./rusty_bot"]
EXPOSE 8000
