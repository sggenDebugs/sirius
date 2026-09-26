FROM rust:1.85 AS sirius-backend
WORKDIR /app/sirius-backend
COPY sirius-backend/Cargo.toml ./
COPY sirius-backend/src ./src
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /app
COPY --from=sirius-backend /app/sirius-backend/target/release/sirius-backend ./
EXPOSE 80
CMD ["./sirius-backend"]