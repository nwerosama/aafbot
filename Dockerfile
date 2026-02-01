FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:4585b6322b40a28877dbe2363c7281dd046d9289300a36f58edc207ed1c8db90
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
