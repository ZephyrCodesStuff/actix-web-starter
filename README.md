<p align="center">
  <img src="https://actix.rs/img/logo.png" alt="Actix logo" width="100">
</p>

<h1 align="center">Actix-Web starter template</h1>
<p align="center">
  A tiny, ready-to-use template for building high-performance APIs with Actix-Web in Rust.
</p>

---

[![GitHub Issues](https://img.shields.io/github/issues/ZephyrCodesStuff/actix-web-starter)](https://github.com/your-username/actix-web-starter/issues)
[![GitHub Stars](https://img.shields.io/github/stars/ZephyrCodesStuff/actix-web-starter)](https://github.com/your-username/actix-web-starter/stargazers)

---

## About

Actix-Web Starter is a tiny template to kickstart your Actix-Web-based Rust API projects. Whether you're building a RESTful API, a microservice, or a web application backend, this tiny template can let you skip the initial hassle of configuring your environment, allowing you to immediately start coding your endpoints.

## Features

- Ready-to-use Actix-Web project structure.
- Support for environment variables using a `.env` file (e.g., `HOST`, `PORT`, `RUST_LOG`).
- Logging set up out of the box.
- Easy-to-extend architecture for adding routes and middleware.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your system.

### Installation

1. Clone this repository:

  ```bash
  git clone https://github.com/your-username/actix-web-starter.git
  cd actix-web-starter
  ```

2. Create a .env file and set your environment variables:

  ```env
  HOST=127.0.0.1
  PORT=8080
  RUST_LOG=info
  ```

3. Build and run the project:

  ```bash
  cargo run
  ```

Your Actix-Web server should now be running at http://localhost:8080.
