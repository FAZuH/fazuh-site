# fazuh-site

Personal portfolio site — built with Dioxus 0.7 fullstack, Tailwind CSS v4.

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/CLI/installation): `cargo install dioxus-cli --locked`
- [Tailwind CSS standalone](https://tailwindcss.com/blog/standalone-cli) binary

### Build & run (Docker)

```bash
docker compose up --build
```

Opens at `http://127.0.0.1:8080`.

### Local dev

```bash
tailwindcss -i input.css -o assets/tailwind.css
dx serve
```

### Refresh live stats

SSHs into lab nodes to count containers, and runs a local `find` for docker-compose stacks. Writes the result directly to `src/container_count.rs`.

```bash
cargo run --bin fetch_stats
```

## License

MIT
