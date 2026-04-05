FROM archlinux:base@sha256:237637c52069930ed7fc76c76f04b6b6b9cf82c5222ae002b7932df983cb69c1
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN pacman-key --init
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /opt/bot
COPY target/release/aaf .
COPY schemas/ schemas/
EXPOSE 9100/tcp
CMD [ "./aaf" ]
