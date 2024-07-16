# Minikube Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/minikube)](https://pkg.fluentci.io/minikube)
[![ci](https://github.com/fluentci-io/services/actions/workflows/minikube.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/minikube.yml)

Minikube service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm minikube start
```

## Functions

| Name   | Description                                  |
| ------ | -------------------------------------------- |
| start  | Start Minikube                               |
| stop   | Stop Minikub                                 |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/minikube@v0.1.1?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: minikube
    args: |
      start
```
