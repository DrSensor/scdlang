FROM rust:latest

LABEL "name"="perf"
LABEL "maintainer"="Fahmi Akbar Wildana <f.a.wildana@gmail.com>"
LABEL "version"="0.1.0"

LABEL "com.github.actions.name"="GitHub Action for Measuring Performance"
LABEL "com.github.actions.description"="Common cli for measuring performance"
LABEL "com.github.actions.icon"="bar-chart-2"
LABEL "com.github.actions.color"="orange"

ENV PATH=/usr/local/bin:/root/.pyenv/bin:/root/.pyenv/shims:${PATH}

RUN wget https://github.com/sharkdp/hyperfine/releases/download/v1.5.0/hyperfine_1.5.0_amd64.deb && \
    dpkg -i hyperfine_1.5.0_amd64.deb && \
    rm -f hyperfine_1.5.0_amd64.deb && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN curl https://pyenv.run | bash && \
    pyenv install 3.7.1 && \
    pyenv global 3.7.1 && \
    apt purge -y python python3 && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY wrap-args.py /usr/local/bin/wrap-args
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD cargo --list