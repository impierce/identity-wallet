FROM rust:latest

EXPOSE 8000

WORKDIR .
COPY . .
RUN cargo install --path .
CMD ["credential_issuer"]

