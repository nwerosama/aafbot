FROM archlinux:base@sha256:40ec92af4b7de7127251038f2e1af7978c1dbc1625e4c7d23b7a89eee05e5a58
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
