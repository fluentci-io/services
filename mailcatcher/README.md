# MailCatcher Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/mailcatcher)](https://pkg.fluentci.io/mailcatcher)
[![ci](https://github.com/fluentci-io/services/actions/workflows/mailcatcher.yml/badge.svg)](https://github.com/fluentci-io/services/actions/workflows/mailcatcher.yml)

MailCatcher service plugin for FluentCI.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm mailcatcher start
```

## Functions

| Name   | Description                                 |
| ------ | --------------------------------------------|
| start  | Start mailcatcher server                    |
| stop   | Stop mailcatcher server                     |

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

dag().call("https://pkg.fluentci.io/mailcatcher@v0.1.0?wasm=1", "start", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: mailcatcher
    args: |
      start
```
