FROM ubuntu:22.04
COPY ./target/release/candles-flows-grpc ./target/release/candles-flows-grpc

ARG app_version
ARG app_compilation_date
ENV APP_VERSION=${app_version}
ENV APP_COMPILATION_DATE=${app_compilation_date}

ENTRYPOINT ["./target/release/trade-log-server"]