FROM ghcr.io/void-linux/void-glibc-busybox:latest@sha256:e6244991ee577689a807f22480fdf6a4400859532cd0fcf6a89b9fd36f5e13be
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN xbps-install -Syu libgcc

WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/

EXPOSE 9100/tcp
CMD [ "./aafbot" ]
