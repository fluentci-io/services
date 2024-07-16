# Stripe Mock Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/stripe-mock)](https://pkg.fluentci.io/stripe-mock)
[![ci](https://github.com/fluentci-io/services/actions/workflows/stripe-mock.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/stripe-mock.yml)

Redis service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm stripe-mock start
```

## Functions

| Name   | Description                                        |
| ------ | -------------------------------------------------- |
| start  | Start Stripe Mock server                                |
| stop   | Stop Stripe Mock server                                 |

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

dag().call("https://pkg.fluentci.io/stripe-mock@v0.1.1?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: stripe-mock
    args: |
      start
```
