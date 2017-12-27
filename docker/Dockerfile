
# Build the frontend.

FROM node as frontend

COPY ./frontend /frontend
RUN cd /frontend && yarn && yarn build

# Build the backend.

FROM ready2order/rust-ci:nightly-2017-12-20 as backend

COPY --from=frontend /frontend/dist /frontend/dist
COPY ./backend /backend

RUN cd /backend && ~/.cargo/bin/cargo build --release

# Actual runtime container.

FROM debian:testing

# Install basic required (linked) dependencies.
RUN apt update && apt install -y openssl libsqlite3-dev sqlite3

# Install dumb-init.
RUN curl -L https://github.com/Yelp/dumb-init/releases/download/v1.2.1/dumb-init_1.2.1_amd64 > /usr/local/bin/dumb-init
RUN chmod +x /usr/local/bin/dumb-init

WORKDIR /translator/
COPY --from=backend /backend/target/release/translator /translator/translator

EXPOSE 80

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/translator/translator"]
