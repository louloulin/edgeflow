# Single binary

You can also run EdgeFlow as a standalone binary using rust's `cargo` or downloading it directly from [https://github.com/luizfonseca/edgeflow/releases](https://github.com/luizfonseca/edgeflow/releases) for you system.

## Cargo

EdgeFlow is a Rust-based proxy service and can be installed as a binary through the published version [on crates.io](https://crates.io/search?q=edgeflow).

To install (and compile) EdgeFlow for your system, first ensure you have the latest Rust version:



### 1. Rust is not installed&#x20;

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Rust is already installed

```bash
rustup update
```

### 3. Install the binary

```bash
cargo install edgeflow
```

You can now  run EdgeFlow as a user binary:

```bash
touch edgeflow.hcl
# add routing configuration to your edgeflow.hcl file

edgeflow -c ./
```



## Downloading the binary

Ensure you are download the right one from the [Releases page on Github](https://github.com/luizfonseca/edgeflow/releases) and once you download it, make sure it has the right permissions to execute, e.g.:

```bash
# Replace {VERSION} with the version you want
# Replace {PLATFORM} with the one for your system
curl -O -L https://github.com/luizfonseca/edgeflow/releases/download/{VERSION}/{PLATFORM}.tar.gz
tar -czvf {PLATFORM}.tar.gz

chmod +x ./edgeflow
```

Once that is done you can check if the binary is functional:

```
edgeflow --help
```
