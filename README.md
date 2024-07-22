<p align="center">
  <img src="./.github/assets/banner.png">
</p>

[![MIT License](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![discord](https://img.shields.io/discord/1132020671262773358?label=discord&logo=discord&color=5865F2)](https://discord.gg/V4U6dPskKc)

This repository contains the source code for [FluentCI](https://fluentci.io) services.
Services are processes that run in the background and provide functionality that you might need to test or operate your application in a CI/CD pipeline.
For example, your CI/CD Pipeline might need to run tests that require access to a database and memory cache.

## 🚀 Usage

Run the following command to start a service locally or on your CI/CD pipeline:

```bash
fluentci run --wasm <service> start
```

Example:

```bash
fluentci run --wasm postgres start
```

## ✨ Features

- **Lightweight**: Services are built using WebAssembly and can run on Linux/MacOS without any additional dependencies or installations, making them lightweight and easy to use.
- **Isolated**: Each service runs in its own isolated environment, ensuring that they do not interfere with existing services or applications on your machine.
- **Customizable**: Services can be customized to suit your needs by modifying the source code, configuration files or environment variables.

## 🧩 Services

| Name                         | Description        |
|------------------------------|--------------------|
| [apache](./apache)           | A powerful and flexible HTTP/1.1 compliant web server |
| [caddy](./caddy)             | Fast and extensible multi-platform HTTP/1-2-3 web server with automatic HTTPS  |
| [cassandra](./cassandra)     | A distributed NoSQL database management system |
| [clickhouse](./clickhouse)   | An open-source column-oriented database management system |
| [cockroachdb](./cockroachdb) | A distributed SQL database built on a transactional and strongly-consistent key-value store |
| [confd](./confd)             | A lightweight configuration management tool |
| [consul](./consul)           | A distributed, highly available, and data center aware solution to connect and configure applications across dynamic, distributed infrastructure |
| [consul-template](./consul-template) | A tool for generating files based on Consul data |
| [couchdb](./couchdb)         | A database that uses JSON for documents, JavaScript for MapReduce indexes, and regular HTTP for its API |
| [daemon](./daemon)           | A plugin that runs a command as a daemon |
| [dragonflydb](./dragonflydb) | A modern replacement for Redis and Memcached |
| [dgraph](./dgraph)           | A distributed, horizontally scalable, and low-latency graph database |
| [dynamodb-local](./dynamodb-local) | A local version of Amazon DynamoDB |
| [elasticmq](./elasticmq/) | A message queue system with the same semantics as Amazon SQS |
| [envconsul](./envconsul)     | A tool that manages environment variables using HashiCorp Consul |
| [etcd](./etcd)               | A distributed key-value store |
| [httpbin](./httpbin)         | HTTP Request & Response Service |
| [influxdb](./influxdb)       | A time series database designed to handle high write and query loads |
| [jaeger](./jaeger)           | A distributed tracing system |
| [mailcatcher](./mailcatcher) | A simple SMTP server that catches all incoming emails |
| [mailhog](./mailhog)         | An email testing tool for developers |
| [mailpit](./mailpit)         | An email and SMTP testing tool with API for developers |
| [mariadb](./mariadb)         | The open source relational database |
| [meilisearch](./meilisearch) | A powerful, fast, open-source, easy to use search engine |
| [memcached](./memcached)     | A high-performance, distributed memory object caching system |
| [minikube](./minikube)       | A tool that makes it easy to run Kubernetes locally |
| [minio](./minio)             | A high performance distributed object storage server |
| [mongodb](./mongodb)         | A general purpose, document-based, distributed database |
| [mysql](./mysql)             | The world's most popular open source database |
| [nats](./nats)              | A simple, secure and high performance messaging system |
| [nginx](./nginx)             | HTTP and reverse proxy server |
| [nsq](./nsq)                 | A realtime distributed messaging platform |
| [opentelemetry-collector](./opentelemetry-collector) | OpenTelemetry Collector superset with additional collectors |
| [postgres](./postgres)       | A database management system that is object-relational |
| [php-fpm](./php)                 | PHP FastCGI Process Manager |
| [quickwit](./quickwit)       | A distributed search engine |
| [rabbitmq](./rabbitmq)       | An open source multi-protocol messaging broker |
| [redis](./redis)             | An in-memory database that persists on disk | 
| [rethinkdb](./rethinkdb)     | An open-source, distributed database built with love |
| [spicedb](./spicedb)         | Open Source, Google Zanzibar-inspired permissions database to enable fine-grained authorization for customer applications |
| [stripe-mock](./stripe-mock/) | A mock HTTP server that responds like the real Stripe API. |
| [temporal](./temporal)       | A distributed, scalable, durable, and highly available orchestration engine |
| [tidb](./tidb)               | A distributed SQL database |
| [typesense](./typesense)     | A fast, typo-tolerant search engine for building delightful search experiences |
| [vault](./vault)             | A tool for managing secrets and protecting sensitive data |
| [zipkin](./zipkin)           | A distributed tracing system |

## 🤝 Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.