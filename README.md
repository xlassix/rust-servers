# Rust Webserver Collection README

Welcome to the Rust Webserver Collection, a curated repository showcasing a variety of web servers implemented in Rust. Each project aims to explore the capabilities and characteristics of different web server libraries available in the Rust ecosystem, providing a comprehensive overview for developers interested in Rust-based web development.

## Projects Overview

### 1. Std-Server

- **Description**: A simple HTTP server built using only Rust's standard library, demonstrating the basics of web server functionality without external dependencies.
- **Features**: Basic request handling, static file serving, and routing.
- **Status**: Completed

### 2. Tide-Server

- **Description**: A lightweight and modular web server utilizing the Tide framework, known for its simplicity and ease of use.
- **Features**: Middleware support, request parsing, and streaming responses.
- **Status**: Completed

### 3. Warp

- **Description**: Warp is a composable web server framework that emphasizes type safety and functional programming patterns.
- **Features**: Request validation, middleware composition, and error handling.
- **Status**: In Progress

### 4. Rocket

- **Description**: Rocket is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety.
- **Features**: Automatic request validation, template rendering, and easy routing.
- **Status**: In Progress

### 5. Actix

- **Description**: Actix is a powerful, pragmatic, and extremely fast web framework for Rust.
- **Features**: High concurrency, asynchronous processing, and a rich ecosystem of crates.
- **Status**: Planned

## Goals

The primary goal of this collection is to explore and compare the strengths and weaknesses of various web server libraries in Rust. By implementing similar functionalities across different frameworks, we aim to provide insights into their performance, ease of use, and community support.

## Todo List

- [ ] Chart response times for each server to compare performance.
- [ ] Add comprehensive test cases to each project to ensure reliability and correctness.
