FROM rust:1.65.0 as builder
WORKDIR /app
COPY app/ .
RUN cargo build --release
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["cargo", "run"]
