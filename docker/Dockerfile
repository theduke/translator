FROM base/archlinux as builder

RUN pacman -Sy && \
    pacman -Sy  --needed --noconfirm curl ca-certificates sqlite3 base-devel nodejs yarn openssl openssl-1.0 && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \
    # rust via rustup
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \

COPY . /translator
RUN cd /translator/frontend && \
    yarn && \
    webpack && \
    cd ../backend && \
    ~/.cargo/bin/cargo build --release


FROM base/archlinux

WORKDIR /translator/
COPY from=builder /translator/backend/target/release/translator ./translator
CMD ["./translator"]
