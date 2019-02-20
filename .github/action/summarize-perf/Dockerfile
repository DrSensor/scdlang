FROM debian:stable-slim

LABEL "name"="summarize-perf" \
      "maintainer"="Fahmi Akbar Wildana <f.a.wildana@gmail.com>" \
      "version"="0.1.0"

LABEL "com.github.actions.name"="GitHub Actions to summarize perf" \
      "com.github.actions.description"="Common tools to summarize all benchmark result by perf action" \
      "com.github.actions.icon"="bar-chart-2" \
      "com.github.actions.color"="green"

ENV PATH="/usr/local/bin:${PATH}"

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
        jq && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY bin /usr/local/bin/
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]