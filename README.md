```bash
cp .env.sample .env
source .env

cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
RUST_LOG=info cargo run
```
