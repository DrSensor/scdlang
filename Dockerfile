FROM alpine

COPY target/x86_64-unknown-linux-musl/release/scrap /usr/bin/

ENTRYPOINT [ "scrap" ]