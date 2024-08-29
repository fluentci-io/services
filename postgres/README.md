# Postgres Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/postgres)](https://pkg.fluentci.io/postgres)
[![ci](https://github.com/fluentci-io/services/actions/workflows/postgres.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/postgres.yml)

Postgres service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm postgres start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start Postgres                              |
| stop   | Stops Postgres                              |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.3"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/postgres@v0.1.7?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: postgres
    args: |
      start
```
