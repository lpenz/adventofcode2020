FROM rust:latest
RUN set -e -x; \
    rustup toolchain install nightly; \
    rustup default nightly; \
    cargo install grcov

COPY entrypoint /
CMD /entrypoint
