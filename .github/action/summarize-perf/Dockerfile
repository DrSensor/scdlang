FROM debian:stable-slim

LABEL "name"="summarize-perf" \
      "maintainer"="Fahmi Akbar Wildana <f.a.wildana@gmail.com>" \
      "version"="0.1.0"

LABEL "com.github.actions.name"="GitHub Actions to summarize perf" \
      "com.github.actions.description"="Common tools to summarize all benchmark result by perf action" \
      "com.github.actions.icon"="bar-chart-2" \
      "com.github.actions.color"="green"

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
        git \
        jq \
        curl \
        ca-certificates \
    && curl -sSL \
    https://github.com/hjson/hjson-go/releases/download/v3.0.0/linux_amd64.tar.gz | \
    tar -xz -C /usr/bin && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY bin /usr/bin/
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]