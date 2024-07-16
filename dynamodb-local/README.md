# DynamoDB Local Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/dynamodb-local)](https://pkg.fluentci.io/dynamodb-local)
[![ci](https://github.com/fluentci-io/services/actions/workflows/dynamodb-local.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/dynamodb-local.yml)

DynamoDB Local service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm dynamodb-local start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start DynamoDB Local Server                       |
| stop   | Stop DynamoDB Local Server                        |

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

dag().call("https://pkg.fluentci.io/dynamodb-local@v0.1.1?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: dynamodb-local
    args: |
      start
```
