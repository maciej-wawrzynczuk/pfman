# pfman

Portfolio manager — a Rust web service for managing transaction logs.

## User guide

`pfman` exposes two HTTP endpoints for managing a transaction log:

### Load a transaction log

```http
POST /transactions
Content-Type: text/csv
```

Upload a CSV transaction log. Each POST **replaces** the entire in-memory log.

CSV format (delimiter `;`, header row required):

```csv
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;182.50;1.00;USD
```

| Field | Type | Format |
| --- | --- | --- |
| `date` | date | `YYYY-MM-DD` |
| `symbol` | string | — |
| `number` | integer | — |
| `price` | decimal | — |
| `commission` | decimal | — |
| `currency` | string | — |

Returns `200 OK` on success, `400 Bad Request` with an error description on failure.

### Query the transaction log

```http
GET /transactions
```

Returns all records from the in-memory log as a JSON array. Returns an empty
array `[]` if no log has been loaded yet.

Example response:

```json
[
  {
    "date": "2024-01-15",
    "symbol": "AAPL",
    "number": 10,
    "price": "182.50",
    "commission": "1.00",
    "currency": "USD"
  }
]
```

`price` and `commission` are serialized as strings to preserve decimal precision.

## Operator guide

### Running the service

```sh
cargo run
# or after building:
./target/release/pfman
```

### Configuration

| Variable | Default | Description |
| --- | --- | --- |
| `PFMAN_LISTEN_ADDR` | `127.0.0.1:8080` | TCP address and port to listen on |
| `RUST_LOG` | — | Log level filter (e.g. `info`, `debug`, `error`) |

Example:

```sh
PFMAN_LISTEN_ADDR=0.0.0.0:3000 RUST_LOG=info ./pfman
```

Startup logs the resolved listen address at `INFO` level. All incoming requests
are logged at `DEBUG` level. Parse errors are logged at `ERROR` level.

## Developer guide

### Prerequisites

- Rust toolchain (stable)
- [`hurl`](https://hurl.dev/) — for HTTP endpoint tests
- `lsof` — used by the `test-http` Makefile target to stop the server

### Build and check

```sh
make check      # cargo check
make clippy     # cargo clippy --pedantic
make build      # cargo build
```

### Testing

```sh
make test       # unit tests (cargo test)
make test-http  # build, start server, run hurl tests, stop server
```

### Architecture

Clean Architecture with inward-only dependencies:

```text
Frameworks & drivers   →  Interface adapters  →  Use cases  →  Entities
(Axum, serde, csv)        (handlers, parsers)    (TransactionLog)  (TransactionEntry)
```

| Layer | Location | Responsibility |
| --- | --- | --- |
| Entities | `src/domain/transaction.rs` | `TransactionEntry` struct |
| Use cases | `src/domain/transaction_log.rs` | `TransactionLog` collection |
| Interface adapters | `src/adapters/csv_parser.rs` | Parse CSV bytes |
| Interface adapters | `src/adapters/json_dto.rs` | Serialize to JSON DTOs |
| Interface adapters | `src/handlers/` | Axum request handlers |
| Frameworks | `src/main.rs` | Router, middleware, server startup |
