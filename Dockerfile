FROM ghcr.io/void-linux/void-glibc-busybox:latest@sha256:a2f034fb573a7758e0c85e41ef9284d3e57932ed9de357c1029fd46600294561
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN xbps-install -Syu libgcc

WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/

EXPOSE 9100/tcp
CMD [ "./aafbot" ]
