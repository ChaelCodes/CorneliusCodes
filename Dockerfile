FROM rust

ARG PORT=8080

ENV PORT $PORT
WORKDIR /usr/src/cornelius-codes
COPY . .

RUN cargo install --path .

EXPOSE $PORT
ENTRYPOINT ["cornelius-codes"]
