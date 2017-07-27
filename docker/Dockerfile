FROM base/archlinux as builder

RUN pacman -Sy && \
    pacman -Sy  --needed --noconfirm curl ca-certificates sqlite3 base-devel nodejs yarn openssl openssl-1.0 && \
    # rust via rustup
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

WORKDIR /translator
COPY ./frontend ./frontend
COPY ./backend ./backend
RUN cd ./frontend && \
    yarn && \
    yarn build && \
    cd ../backend

RUN cd backend && ~/.cargo/bin/cargo build --release

FROM base/archlinux

WORKDIR /translator/
COPY --from=builder /translator/backend/target/release/translator /translator/translator

EXPOSE 80

CMD ["/translator/translator"]
