FROM archlinux:base@sha256:b4df475619469581537637ceaace6075f4912ea1cd6db6b99d463e3a72969d04
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN pacman-key --init && pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /opt/bot
COPY target/release/aafbot .
COPY schemas/ schemas/
EXPOSE 9100/tcp
CMD [ "./aafbot" ]
