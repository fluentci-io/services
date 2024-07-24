# Neo4j Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/neo4j)](https://pkg.fluentci.io/neo4j)
[![ci](https://github.com/fluentci-io/services/actions/workflows/neo4j.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/neo4j.yml)

Neo4j service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm neo4j start
```

## Functions

| Name   | Description                    |
| ------ | ------------------------------ |
| start  | Start neo4j                    |
| stop   | Stop neo4j                     |

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

dag().call("https://pkg.fluentci.io/neo4j@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: neo4j
    args: |
      start
```
