# TiDB Local Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/tidb)](https://pkg.fluentci.io/tidb)
[![ci](https://github.com/fluentci-io/services/actions/workflows/tidb.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/tidb.yml)

TiDB service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm tidb start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start TiDB Local Server                       |
| stop   | Stop TiDB Local Server                        |

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

dag().call("https://pkg.fluentci.io/tidb@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: tidb
    args: |
      start
```
