FROM quay.io/fedora/fedora:latest as builder

RUN dnf update -y && \
    dnf install gcc-c++ clang llvm-devel openssl-devel -y && \
    dnf clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH $HOME/.cargo/bin:$PATH

WORKDIR /usr/src/app
COPY ./  /usr/src/app

RUN $HOME/.cargo/bin/cargo install --path . --locked --verbose

## runtime image configuration
FROM quay.io/fedora/fedora:latest as runtime

RUN dnf update -y && \
    dnf clean all

COPY --from=builder /root/.cargo/bin/jira-gitlab-validating-webhook  /usr/local/bin/jira-gitlab-validating-webhook
ENTRYPOINT ["/usr/local/bin/jira-gitlab-validating-webhook"]