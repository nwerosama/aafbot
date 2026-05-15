FROM archlinux:base@sha256:ceac417c19645d21630c120fa123942aa1fc5988faab14e67222013cb11f31bb
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
