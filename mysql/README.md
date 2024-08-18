# MySQL Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/mysql)](https://pkg.fluentci.io/mysql)
[![ci](https://github.com/fluentci-io/services/actions/workflows/mysql.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/mysql.yml)

MySQL service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm mysql start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start MySQL                                 |
| stop   | Stops MySQL                                 |

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

dag().call("https://pkg.fluentci.io/mysql@v0.1.4?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: mysql
    args: |
      start
```
