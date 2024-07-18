# Daemon Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/daemon)](https://pkg.fluentci.io/daemon)
[![ci](https://github.com/fluentci-io/services/actions/workflows/daemon.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/daemon.yml)

Daemon service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm daemon start
```

## Functions

| Name   | Description                                    |
| ------ | ---------------------------------------------- |
| start  | Start daemon                               |
| stop   | Stop daemon                                 |

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

dag().call("https://pkg.fluentci.io/daemon@v0.1.0?wasm=1", "start", vec!["demo", "pkgx bunx serve -p 4000"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: daemon
    args: |
      start demo dpkgx bunx serve -p 4000
```
