FROM alpine AS grapheasy

RUN apk add make \
            perl-app-cpanminus \ 
            perl-par-packer \
    --no-cache && \
    cpanm Graph::Easy \
          Getopt::ArgvFile \
    --no-wget
RUN mkdir /dist && \
    pp $(which graph-easy) --output /dist/graph-easy

RUN /dist/graph-easy --version || [ $? == 2 ] && exit 0
# I know 👆 is old module but why it return 2 instead of 0 🤣


FROM node:alpine AS smcat

RUN npm pack state-machine-cat && \
    tar xvzf *.tgz && cd package && \
    npm install pump once wrappy \
                fast-deep-equal uri-js \
                json-schema-traverse \
                fast-json-stable-stringify \
                end-of-stream \
    && npm install
RUN cd package && \
    npx pkg bin/smcat --output /dist/smcat

RUN /dist/smcat --version


FROM node:alpine

LABEL version="0.2.1" \
      repository="https://github.com/drsensor/scdlang" \
      homepage="https://drsensor.github.io/scdlang" \
      maintainer="Fahmi Akbar Wildana <sensorfied@gmail.com>"
COPY README.md LICENSE CODE_OF_CONDUCT.md /

COPY target/x86_64-unknown-linux-musl/release/scrap /usr/bin/
COPY --from=smcat /dist/* /usr/bin/
COPY --from=grapheasy /dist/* /usr/bin/
RUN apk add --no-cache graphviz

ENTRYPOINT [ "scrap" ]
# CMD [ "repl", "--interactive", ] # TODO: uncomment when prompt to select --format is implemented