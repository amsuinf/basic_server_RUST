# basic_server_RUST

Every Rust journey has a first server. This is mine.

Before diving into distributed systems, Raft consensus, and production-grade infrastructure, I wanted to build something deliberately simple — a server that does exactly one thing and does it cleanly. No shortcuts, no tutorials copy-pasted blindly. Just Rust, Axum, Tokio, and me figuring out how the pieces fit together.

This is that project.

---

## What It Does

Starts an async HTTP server on port `3000` and responds to `GET /` with `Hello, World!`.

That's it. And that's the point.

---

## Why It Exists

Learning Rust properly means understanding the fundamentals before layering complexity on top. This project was about getting comfortable with:

- How to structure a Rust project with a clean separation between `lib` and `bin`
- How `tokio` drives async execution under the hood
- How `axum` wires together routers, routes, and handlers
- How a `TcpListener` binds to an address and hands off connections to the framework

The code you see here isn't minimal because I was being lazy. It's minimal because I wanted every line to be something I actually understood.

---

## Project Structure

```
src/
├── main.rs       — entry point, kicks off the async runtime
├── lib.rs        — module declarations, exposes the crate as `pkg`
├── app.rs        — server setup: binds to 0.0.0.0:3000 and starts serving
├── router.rs     — route definitions
└── handler.rs    — request handlers
```

The separation between `app`, `router`, and `handler` is intentional. Even for a project this small, keeping these concerns apart makes the mental model cleaner — and makes it easier to grow later.

---

## Stack

| | |
|---|---|
| Language | Rust (Edition 2024) |
| Web Framework | [Axum](https://github.com/tokio-rs/axum) 0.8 |
| Async Runtime | [Tokio](https://tokio.rs/) 1.x (full features) |

---

## Running It

Make sure you have Rust installed. If not, get it from [rustup.rs](https://rustup.rs/).

```bash
git clone https://github.com/amsuinf/basic_server_RUST.git
cd basic_server_RUST
cargo run
```

Server starts on `http://0.0.0.0:3000`.

Hit it:

```bash
curl http://localhost:3000
# Hello, World!
```

---

## What's Next

This server is a foundation, not a destination. The natural next steps from here:

- Add more routes and a proper routing layer
- Introduce middleware — logging, request tracing
- Connect a database
- Add error handling that goes beyond `unwrap()`
- Wire up a health check endpoint (the stub is already in `handler.rs`)

---

## A Note

If you're also learning Rust and stumbled on this — start here. Not with the most complex thing you can find. Build something small enough that you can hold the entire mental model in your head at once, understand every line, and feel what it's like when it actually works.

That feeling is worth a lot.

---

*Built by [Asif](https://github.com/amsuinf)*
