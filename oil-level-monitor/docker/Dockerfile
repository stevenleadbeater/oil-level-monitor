FROM alpine:latest
RUN mkdir /opt/service
WORKDIR /opt/service
COPY oil-level-monitor .
COPY *.toml ./
EXPOSE 8120
CMD ["/opt/service/oil-level-monitor"]