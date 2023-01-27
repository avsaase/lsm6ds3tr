FROM fedora:36

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN dnf install -y git wget curl clang cmake openssl-devel && \
    dnf clean all

ENV USER user
RUN groupadd "${USER}" --gid 1000
RUN useradd "${USER}" --uid 1000 --gid 1000 --groups "wheel" --shell "/bin/bash" --create-home --no-log-init
RUN echo "%wheel   ALL=(ALL:ALL) NOPASSWD: ALL" >> /etc/sudoers
USER "${USER}"
ENV HOME="/home/${USER}"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
WORKDIR "${HOME}"

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain=1.67

# hadolint ignore=SC2016
RUN echo 'source $HOME/.cargo/env' >> "${HOME}/.bashrc"

RUN rustup update stable nightly

RUN rustup component add rustfmt rust-analysis clippy

RUN cargo install cargo-udeps
