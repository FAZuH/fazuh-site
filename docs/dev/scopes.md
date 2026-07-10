# Commit scopes

| Scope   | When to use | Touches |
|---------|-------------|---------|
| `ui`    | Components, CSS, layout, assets | `src/components/*`, `src/app.rs`, `input.css`, `assets/` |
| `server`| Backend logic | `server.rs`, `smtp.rs`, `config.rs`, `logging.rs`, `rate_limit.rs`, `validation.rs`, `server_stats.rs` |
| `ops`   | Docker, CI/CD, deps, docs | `Dockerfile`, `docker-compose.yml`, `.github/`, `Cargo.*`, `docs/*`, `README.md`, `src/bin/` |
