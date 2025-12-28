FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:f7047b912073aba008a42602920728e3fd604f4a1b50ae35babd64012eba5b3e
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
