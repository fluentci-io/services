# Envconsul Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/envconsul)](https://pkg.fluentci.io/envconsul)
[![ci](https://github.com/fluentci-io/services/actions/workflows/envconsul.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/envconsul.yml)

Envconsul service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm envconsul start
```

## Functions

| Name   | Description                                    |
| ------ | ---------------------------------------------- |
| start  | Start envconsul                                |
| stop   | Stop envconsul                                 |

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

dag().call("https://pkg.fluentci.io/envconsul@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: envconsul
    args: |
      start
```
