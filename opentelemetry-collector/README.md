# Opentelemetry Collector Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/opentelemetry-collector)](https://pkg.fluentci.io/opentelemetry-collector)
[![ci](https://github.com/fluentci-io/services/actions/workflows/opentelemetry-collector.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/opentelemetry-collector.yml)

Opentelemetry service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm opentelemetry-collector start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start Opentelemetry Collector               |
| stop   | Stop Opentelemetry Collector                |

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

dag().call("https://pkg.fluentci.io/opentelemetry-collector@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: opentelemetry-collector
    args: |
      start
```
