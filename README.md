# fazuh-site

Personal portfolio — Dioxus 0.7 fullstack, Tailwind CSS v4, JetBrains Mono, SMTP.

## Sections

- Hero (avatar + Tmplr ASCII wordmark + neofetch-style stats)
- About (3 bracket bullets + GitHub link + email)
- Skills (filesystem tree with shields.io badges)
- Projects (expandable tree: notable, smaller-projects, course-work)
- Contact (name + email + message → SMTP via lettre)

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

### Refresh container count

```bash
cargo run --bin fetch_stats > src/container_count.rs
```

## CI

Managed via [project-ops](https://github.com/FAZuH/project-ops). Pushing to `release` builds and pushes to `ghcr.io/fazuh/fazuh-site:latest`.

## License

MIT
