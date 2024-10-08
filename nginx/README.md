# Nginx Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/nginx)](https://pkg.fluentci.io/nginx)
[![ci](https://github.com/fluentci-io/services/actions/workflows/nginx.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/nginx.yml)

Nginx service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm nginx start
```

## Functions

| Name   | Description                                        |
| ------ | -------------------------------------------------- |
| start  | Start Nginx Server                                 |
| stop   | Stops Nginx Server                                 |

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

dag().call("https://pkg.fluentci.io/nginx@v0.1.1?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: nginx
    args: |
      start
```
