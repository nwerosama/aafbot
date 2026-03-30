FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:8435d30c1462f94345c3894ebbeaeecd658fcb284aa41b0bcc5f88728933447f
LABEL org.opencontainers.image.source="https://github.com/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN pacman-key --init
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /bot
COPY --from=base /builder/target/release/aaf .
COPY --from=base /builder/schemas/ schemas/
EXPOSE 9100/tcp
CMD [ "./aaf" ]
