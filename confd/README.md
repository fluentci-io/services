# Confd Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/confd)](https://pkg.fluentci.io/confd)
[![ci](https://github.com/fluentci-io/services/actions/workflows/confd.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/confd.yml)

Confd service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm confd start
```

## Functions

| Name   | Description                                    |
| ------ | ---------------------------------------------- |
| start  | Start confd                                    |
| stop   | Stop confd                                     |

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

dag().call("https://pkg.fluentci.io/confd@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: confd
    args: |
      start
```
