
FROM rust:1.51.0
WORKDIR /app
COPY . /app

RUN cargo build

ENV DATABASE_URL="postgres://rust-todo-db.cl0thk4vxcz7.us-east-2.rds.amazonaws.com:5432/rust_todo?user=postgres&password=StrongMaster123"
EXPOSE 5000
ENTRYPOINT ["target/debug/apie"]