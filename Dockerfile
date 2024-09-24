FROM rust:1.81

EXPOSE 3000

WORKDIR /usr/src/profile_backend
COPY . .

RUN cargo install --path .
ENTRYPOINT ["profile_backend"]
