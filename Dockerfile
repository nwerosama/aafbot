FROM ghcr.io/void-linux/void-glibc-busybox:latest@sha256:931d24baffdbbc3a8d080f1dd9182df5b49d9003345607ce28a7b8ea0c6bd4c0
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN xbps-install -Syu libgcc

WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/

EXPOSE 9100/tcp
CMD [ "./aafbot" ]
