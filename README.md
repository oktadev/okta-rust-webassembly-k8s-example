# Rust WebAssembly on Kubernetes with Krustlet

This is an example Rust WebAssembly(Wasm) workload for Kubernetes.

You will use Krustlet to run a Wasm workload written in Rust on Kubernetes today.

**Prerequisites**

- [Docker](https://docs.docker.com/engine/install/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/)
- [kind](https://kind.sigs.k8s.io/docs/user/quick-start/#installation) or another local kubernetes distribution
- [Rust toolkit](https://www.rust-lang.org/learn/get-started) (Includes rustup, rustc, and cargo)

Follow the blog post ["Containerless! Run WebAssembly Workloads written in Rust on Kubernetes"](https://developer.okta.com/blog/2022/01/28/webassembly-on-kubernetes-with-rust) for further instruction on setting up the cluster and running the workload.

# Build

You can build the app for Wasm using below command.

```bash
# add WASI target
rustup target add wasm32-wasi

# build
cargo build --release --target wasm32-wasi
```

# Push to OCI registry

```bash
wasm-to-oci push target/wasm32-wasi/release/rust-wasm.wasm ghcr.io/<your GitHub user>/rust-wasm:latest
```

## License

Apache 2.0, see [LICENSE](LICENSE).
