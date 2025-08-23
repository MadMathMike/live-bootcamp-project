## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
./docker.sh
```

visit http://localhost:8000 and http://localhost:3000

## Dependencies
```bash
docker run --name ps-db -e POSTGRES_PASSWORD=[YOUR_POSTGRES_PASSWORD] -p 5432:5432 -d postgres:15.2-alpine
docker run --name redis-db -p "6379:6379" -d redis:7.0-alpine
```