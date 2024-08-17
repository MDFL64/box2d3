Building this is quite obnoxious. I have gotten it working on two platforms:

For web, make sure emscripten and cmake are installed and run ./run_web.sh. I only got this to work on Linux (WSL). If you want to serve the files locally to test them, run `cargo install basic-http-server`.

Fow windows (might work for other platforms), run:
```sh
cargo install cargo-vcpkg
cargo vcpkg build
cargo run --release
```
