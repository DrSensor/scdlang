FROM scratch

LABEL version="0.2.1" \
      repository="https://github.com/drsensor/scdlang" \
      homepage="https://drsensor.github.io/scdlang" \
      maintainer="Fahmi Akbar Wildana <sensorfied@gmail.com>"
COPY README.md LICENSE CODE_OF_CONDUCT.md /

COPY target/x86_64-unknown-linux-musl/release/scrap /usr/bin/

ENTRYPOINT [ "scrap" ]
# CMD [ "repl", "--interactive", ] # TODO: uncomment when prompt to select --format is implemented