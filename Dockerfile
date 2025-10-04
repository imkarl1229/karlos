FROM rust:latest

RUN apt-get update && apt-get install -y \
    xorriso make build-essential ovmf binutils openssl efitools\
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

CMD ["bin/bash"]