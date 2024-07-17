# Consul Template Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/consul-template)](https://pkg.fluentci.io/consul-template)
[![ci](https://github.com/fluentci-io/services/actions/workflows/consul-template.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/consul-template.yml)

Consul Template service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm consul-template start
```

## Functions

| Name   | Description                                    |
| ------ | ---------------------------------------------- |
| start  | Start consul-template                          |
| stop   | Stop consul-template                           |

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

dag().call("https://pkg.fluentci.io/consul-template@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: consul-template
    args: |
      start
```
