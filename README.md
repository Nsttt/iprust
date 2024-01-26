# iprust

`iprust` is a simple web application built with Actix-Web in Rust. It's designed to return the public IP address of the client making a request to the server.

## Features

- Responds with the client's public IP address.
- Built with Actix-Web, a powerful and efficient web framework for Rust.
- Easy to deploy with Docker and Docker Compose.

## Requirements

- Rust
- Cargo
- Docker (for containerization)

## Getting Started

To get started with `iprust`, clone the repository and navigate into the project directory:

```bash
git clone https://github.com/yourusername/iprust.git
cd iprust
```

### Running Locally

You can run the application locally using Cargo:

```bash
cargo run
```

The server will start and listen for requests. You can test it by accessing `http://localhost:3000` in your browser or using a tool like `curl`.

### Running with Docker

To build and run the application using Docker and Docker Compose, use the following commands:

```bash
docker compose build
docker compose up
```

This will start the application in a Docker container. The service will be available on the port specified in your `.env` file.

## Configuration

The application can be configured through environment variables. Create a `.env` file in the project root with the following content:

```env
PORT=3000
RUST_LOG=info
```

- `PORT`: The port number on which the application will listen.
- `RUST_LOG`: The logging level for the application.
