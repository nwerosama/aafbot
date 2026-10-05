FROM ghcr.io/void-linux/void-glibc-busybox:latest@sha256:125d486634528bd55ef089c487c9108e75cf003fd002cf5ba86efc2800fdcf92
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN xbps-install -Syu libgcc

WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/

EXPOSE 9100/tcp
CMD [ "./aafbot" ]
