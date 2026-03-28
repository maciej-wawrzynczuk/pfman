# Plan: Transaction log services

## Execution order

```text
Task 1 (scaffolding)
  └─ Task 2 (TransactionEntry)
       ├─ Task 4 (CSV parser) ──────────────────┐
       ├─ Task 5 (JSON DTO) ───────────────────┐ │
       └─ Task 3 (TransactionLog)              │ │
            └─ Task 6 (AppState)               │ │
                 ├─ Task 7 (POST handler) ←────┘ │
                 └─ Task 8 (GET handler)  ←───────┘
                      └─ Task 9 (router + main)
                           └─ Task 10 (hurl tests + Makefile)
```

Parallel groups:

- **A** (after Task 2): Tasks 4 and 5
- **B** (after Task 6): Tasks 7 and 8

---

## Task 1 — Project scaffolding

**Parallelism:** First. All other tasks depend on it.

**Changed files:**

- `Cargo.toml` — add dependencies:
  - `axum`, `tower`, `tokio` (features: full)
  - `serde` (features: derive), `serde_json`, `serde_with`
  - `csv`
  - `rust_decimal` (features: serde)
  - `chrono` (features: serde)
  - `tracing`, `tracing-subscriber` (features: env-filter)
  - `tower-http` (features: trace)
- `src/main.rs` — stub: `#[tokio::main]` with empty router, no routes yet
- `src/domain/mod.rs` — empty, declares future submodules
- `src/adapters/mod.rs` — empty, declares future submodules
- `src/handlers/mod.rs` — empty, declares future submodules
- `Makefile` — targets: `check`, `clippy`, `test`, `run`

**Tests:** none

---

## Task 2 — Entity: `TransactionEntry`

**Parallelism:** Requires Task 1.

**Changed files:**

- `src/domain/transaction.rs` — new file:
  - `pub struct TransactionEntry` with all fields `pub`
  - `#[derive(Debug, Clone, serde::Deserialize)]`
  - Field types: `date: NaiveDate`, `symbol: String`, `number: i64`,
    `price: Decimal`, `commission: Decimal`, `currency: String`
  - `#[serde(rename = "...")]` attributes matching CSV headers
- `src/domain/mod.rs` — declare `pub mod transaction`

**Tests (unit, in `src/domain/transaction.rs`):**

- Deserialize a valid CSV row string → `TransactionEntry` with correct fields
- Deserialize row with invalid date format → error
- Deserialize row with invalid decimal in `price` → error

---

## Task 3 — Use case: `TransactionLog`

**Parallelism:** Requires Task 2.

**Changed files:**

- `src/domain/transaction_log.rs` — new file:
  - `pub struct TransactionLog { entries: Vec<TransactionEntry> }`
  - `impl TransactionLog`: `pub fn new(entries: Vec<TransactionEntry>) -> Self`
  - `pub fn iter(&self) -> impl Iterator<Item = &TransactionEntry>`
  - `pub fn len(&self) -> usize`
  - `pub fn is_empty(&self) -> bool`
- `src/domain/mod.rs` — declare `pub mod transaction_log`

**Tests (unit):**

- `new(entries)` then `iter()` yields all entries in insertion order
- `is_empty()` returns `true` on empty log, `false` on non-empty
- `len()` returns correct count

---

## Task 4 — Interface adapter: CSV parser

**Parallelism:** Requires Task 2. Parallel with Task 5.

**Changed files:**

- `src/adapters/csv_parser.rs` — new file:
  - `pub fn parse(input: &[u8]) -> Result<Vec<TransactionEntry>, csv::Error>`
  - Configures `csv::ReaderBuilder` with `delimiter(b';')` and `has_headers(true)`
  - Deserializes each row into `TransactionEntry` via serde
- `src/adapters/mod.rs` — declare `pub mod csv_parser`

**Tests (unit, covering POST test cases from design):**

- TC1: valid multi-row CSV → `Ok(vec![..])` with correct entry count
- TC2: valid single-row CSV → `Ok(vec![..])` with one entry
- TC3: header-only CSV → `Ok(vec![])` (empty)
- TC4: comma-delimited CSV → `Err`
- TC5: missing column → `Err`
- TC6: invalid decimal in `price` → `Err`
- TC7: invalid date format → `Err`
- TC8: empty input → `Err`

---

## Task 5 — Interface adapter: JSON DTO

**Parallelism:** Requires Task 2. Parallel with Task 4.

**Changed files:**

- `src/adapters/json_dto.rs` — new file:
  - `pub struct TransactionEntryDto` with `#[derive(serde::Serialize)]`
  - `price` and `commission` as `String`
    (use `#[serde(with = "rust_decimal::serde::str")]`)
  - `date` as `String` in `%Y-%m-%d`
    (use `#[serde(with = "...")]` or manual `Display`)
  - `impl From<&TransactionEntry> for TransactionEntryDto`
- `src/adapters/mod.rs` — declare `pub mod json_dto`

**Tests (unit, covering GET test case 4 from design):**

- Convert `TransactionEntry` with known decimal and date values →
  serialized JSON string contains exact string representations
  (e.g., `"price":"182.50"` not `"price":182.5`)

---

## Task 6 — Application state

**Parallelism:** Requires Task 3.

**Changed files:**

- `src/state.rs` — new file:
  - `pub type AppState = Arc<RwLock<Option<TransactionLog>>>`
  - `pub fn initial_state() -> AppState`
- `src/main.rs` — declare `mod state`

**Tests:** none (pure type alias and constructor)

---

## Task 7 — POST `/transactions` handler

**Parallelism:** Requires Tasks 4 and 6. Parallel with Task 8.

**Changed files:**

- `src/handlers/post_transactions.rs` — new file:
  - `pub async fn handler(State(state): State<AppState>, body: Bytes) -> impl IntoResponse`
  - Calls `csv_parser::parse(&body)`
  - On error: `tracing::error!("{}", e)`, returns `StatusCode::BAD_REQUEST`
    with error message body
  - On success: acquires write lock, replaces log with
    `Some(TransactionLog::new(entries))`, returns `StatusCode::OK`
- `src/handlers/mod.rs` — declare `pub mod post_transactions`

**Tests:** none in unit (top-level handler function — no testable logic per rules).
Covered by hurl in Task 10.

---

## Task 8 — GET `/transactions` handler

**Parallelism:** Requires Tasks 5 and 6. Parallel with Task 7.

**Changed files:**

- `src/handlers/get_transactions.rs` — new file:
  - `pub async fn handler(State(state): State<AppState>) -> impl IntoResponse`
  - Acquires read lock; if `None`, returns `Json(Vec::<TransactionEntryDto>::new())`
  - Otherwise maps entries via `TransactionEntryDto::from`, returns `Json(vec)`
- `src/handlers/mod.rs` — declare `pub mod get_transactions`

**Tests:** none in unit (top-level handler function). Covered by hurl in Task 10.

---

## Task 9 — Router, middleware, and main

**Parallelism:** Requires Tasks 7 and 8.

**Changed files:**

- `src/main.rs` — full implementation:
  - Init `tracing_subscriber::fmt()` with `env_filter` from `RUST_LOG`
  - Read `PFMAN_LISTEN_ADDR` env var, default `127.0.0.1:8080`
  - Parse as `SocketAddr`; on failure log error and exit
  - Log resolved address at `INFO` level
  - Create `AppState` via `state::initial_state()`
  - Build router: `POST /transactions`, `GET /transactions`
  - Add `TraceLayer` from `tower_http` for DEBUG-level request logging
  - Bind and serve

**Tests:** none (top-level)

---

## Task 10 — Hurl endpoint tests and Makefile targets

**Parallelism:** Requires Task 9.

**Changed files:**

- `tests/post_transactions.hurl` — POST test cases 1–8 from design
- `tests/get_transactions.hurl` — GET test cases 1–4 from design
- `Makefile` — `test-http` target: starts server in background,
  waits for it to be ready, runs hurl files, stops server

**Tests:** the hurl files are the tests
