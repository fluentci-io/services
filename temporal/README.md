# Temporal Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/temporal)](https://pkg.fluentci.io/temporal)
[![ci](https://github.com/fluentci-io/services/actions/workflows/temporal.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/temporal.yml)

Temporal service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm temporal start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start Temporal Server                       |
| stop   | Stop Temporal Server                        |

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

dag().call("https://pkg.fluentci.io/temporal@v0.1.2?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: temporal
    args: |
      start
```