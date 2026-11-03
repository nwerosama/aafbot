FROM ghcr.io/void-linux/void-glibc-busybox:latest@sha256:03ee2c5b96a4ebde760986fbf5a30a6881821afe3caecfec0b649b2334fd54f9
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN xbps-install -Syu libgcc

WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/

EXPOSE 9100/tcp
CMD [ "./aafbot" ]
