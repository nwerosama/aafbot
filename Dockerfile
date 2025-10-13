FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:287bf95d97e4f952a94a1f4a83008c6a547405bacc44173bda151231a3c843aa
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
