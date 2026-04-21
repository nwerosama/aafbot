FROM archlinux:base@sha256:5ba8bb318666baef4d33afefc0e65db80f38b23503cb8e7b150d315cc2d4d5da
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
