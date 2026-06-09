FROM archlinux:base@sha256:a1f78e6a19f19d0f6cd9bf35ec59de31fe0bdeff6a94e968be0276ca255f14cd
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN pacman-key --init && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/
EXPOSE 9100/tcp
CMD [ "./aafbot" ]
