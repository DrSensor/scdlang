ARG base_image
FROM ${base_image}

LABEL version="0.1.1" \
      repository="https://github.com/drsensor/scdlang" \
      homepage="https://drsensor.github.io/scdlang" \
      maintainer="Fahmi Akbar Wildana <sensorfied@gmail.com>"
COPY ../LICENSE ../README.md ../CODE_OF_CONDUCT.md /

COPY ../target/x86_64-unknown-linux-musl/release/scrap /usr/bin/

ENTRYPOINT [ "scrap" ]