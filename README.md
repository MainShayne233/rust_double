# Rust Double

Simple Rust web server that will double a number.

## Run

[Install Rust](https://www.rust-lang.org/en-US/install.html)

```bash
# clone and enter repo
git clone https://github.com/MainShayne233/rust_double
cd rust_double

# run the application
cargo run


```bash
# hit the server with a POST request and JSON payload
curl -H "Content-Type: application/json" -d '{"number":2}' -X POST '127.0.0.1:8080/double'
#=> {"number":4}
```
