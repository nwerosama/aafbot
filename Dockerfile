FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:9a72b5e3c1675683016cb065f513deea7c65836cb5bd22b88c89353098faa40f
LABEL org.opencontainers.image.source="https://ghcr.io/nwerosama/aafbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /bot
COPY --from=base /builder/target/release/aaf .
COPY --from=base /builder/schemas/ schemas/
EXPOSE 9100/tcp
CMD [ "./aaf" ]
