FROM rust:1.37

WORKDIR /usr/src/hex
COPY . .

RUN ["rustup", "update", "nightly"]
RUN ["rustup", "default", "nightly"]
RUN cargo install --path .

RUN ["apt", "update"]
RUN ["apt", "install", "-y", "nginx"]
COPY nginx.conf /etc/nginx/nginx.conf

CMD ["./run.sh"]
