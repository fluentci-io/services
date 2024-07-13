<p align="center">
  <img src="./.github/assets/banner.png">
</p>

![MIT License](https://img.shields.io/badge/license-MIT-green.svg)
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
| [dragonflydb](./dragonflydb) | A modern replacement for Redis and Memcached | 
| [httpbin](./httpbin)         | HTTP Request & Response Service |
| [mariadb](./mariadb)         | The open source relational database |
| [meilisearch](./meilisearch) | A powerful, fast, open-source, easy to use search engine |
| [mysql](./mysql)             | The world's most popular open source database |
| [nginx](./nginx)             | HTTP and reverse proxy server |
| [postgres](./postgres)       | A database management system that is object-relational |
| [php-fpm](./php)                 | PHP FastCGI Process Manager |
| [rabbitmq](./rabbitmq)       | An open source multi-protocol messaging broker |
| [redis](./redis)             | An in-memory database that persists on disk | 
| [temporal](./temporal)       | A distributed, scalable, durable, and highly available orchestration engine |

## 🤝 Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](./CONTRIBUTING.md) file.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.