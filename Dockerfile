FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:6de1a7bfb793f8d9e24a7d573234f60d011d16db546de5bd777b75707fd4aff4
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
