FROM rust:1.82-bullseye

ARG UID=1000
ARG USER=dev

RUN apt update && \
    apt install -y sudo git vim curl openssl && \
    # Create non root user
    groupadd -g ${UID} ${USER} && \
    useradd -m -u ${UID} -g ${UID} ${USER} -s /bin/bash && \
    echo "${USER} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers && \
    rustup component add clippy rustfmt
