# serve\_md
serve\_md is a simple web server written in Rust that takes a single markdown file and hosts a basic static html page.
## Installation
The package isn't available on any package manager, so you must download it from the releases or compile it yourself. Here are the steps to get it from the releases:
```bash
wget https://github.com/GhelloZ/serve_md/releases/download/v1.2.0/serve_md -o serve_md
chmod +x serve_md
mv serve_md /usr/local/bin      # optional, useful if you want to call the program from anywhere in the system
```
The releases on github are statically compiled binaries, if you want a slightly smaller executable you should build a dynamically linked binary following the instructions below

## Build instructions
It's a simple rust project, so you just have to download the repo and run `cargo build`
```bash
git clone https://github.com/GhelloZ/serve_md.git   # Download the repo
cd serve_md                 # Move into the cloned repo
cargo build                 # It will build a debug version in target/debug/serve_md
cargo build --release       # Will take a bit longer, but the package will be slightly smaller and faster
# if you want to build a no deps version to run it in docker container or minimal systems (like alpine), you want to build a statically linked version
rustup target add x86_64-unkown-linux-musl
# install musl from your repos
cargo build --release --target x86_64-unknown-linux-musl
```
## Use
If you just want to do some quick testing, move into the cloned repo and execute `cargo run -- /path/to/file.md`. This will host the webpage binded on 0.0.0.0:3000
If you already downloaded the latest release or compiled the project just run `serve_md /path/to/file.md`.
Additionally, you can set a port and an IP address to bind the webpage to a specific interface like this: 
```bash
serve_md --address 192.168.0.100 --port 8080 /path/to/file
```
To see a full list of features such as setting colors for text and background use `serve_md -h`
