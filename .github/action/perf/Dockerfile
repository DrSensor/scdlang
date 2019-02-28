FROM python:slim AS helper

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
        binutils \
    && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY wrap-args.py .
RUN pip install pyinstaller && \
    pyinstaller wrap-args.py --onefile --distpath /bin

# ----------------- Github Action ------------------------
FROM rust:slim

LABEL "name"="perf" \
      "maintainer"="Fahmi Akbar Wildana <f.a.wildana@gmail.com>" \
      "version"="0.1.0"

LABEL "com.github.actions.name"="GitHub Action for Measuring Performance" \
      "com.github.actions.description"="Common cli for measuring performance" \
      "com.github.actions.icon"="clock" \
      "com.github.actions.color"="orange"

ADD https://github.com/sharkdp/hyperfine/releases/download/v1.5.0/hyperfine_1.5.0_amd64.deb \
    https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64 \
    /tmp/

RUN chmod +x /tmp/* && \
    dpkg -i /tmp/hyperfine_1.5.0_amd64.deb && \
    mv /tmp/jq-linux64 /usr/bin/jq && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY --from=helper /bin/wrap-args /usr/bin/
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD cargo --list