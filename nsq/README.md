# Nsq Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/nsq)](https://pkg.fluentci.io/nsq)
[![ci](https://github.com/fluentci-io/services/actions/workflows/nsq.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/nsq.yml)

Nsq service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm nsq start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start nsq server                       |
| stop   | Stop nsq server                        |

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

dag().call("https://pkg.fluentci.io/nsq@v0.1.1?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: nsq
    args: |
      start
```
