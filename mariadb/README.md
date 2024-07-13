# MariaDB Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/mariadb)](https://pkg.fluentci.io/mariadb)
[![ci](https://github.com/fluentci-io/services/actions/workflows/mariadb.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/mariadb.yml)

MariaDB service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm mariadb start
```

## Functions

| Name   | Description                                  |
| ------ | -------------------------------------------- |
| start  | Start MariaDB                                |
| stop   | Stop MariaDB                                 |

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

dag().call("https://pkg.fluentci.io/mariadb@v0.1.2?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: mariadb
    args: |
      start
```
