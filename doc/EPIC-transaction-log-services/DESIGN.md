# Design: Transaction log services

## System context

`pfman` is a greenfield Rust web service. This epic introduces its first
two HTTP endpoints and establishes the foundational architecture:
in-memory transaction log loaded from CSV, served as JSON.

There is no existing system to integrate with. All components are
introduced here.

## Dependencies

Following Clean Architecture, dependencies point inward only.

```text
Frameworks & drivers   →  Interface adapters  →  Use cases  →  Entities
(Axum, serde, csv)        (handlers, parsers)    (TransactionLog)  (TransactionEntry)
```

| Layer | Component | Depends on |
| --- | --- | --- |
| Entities | `TransactionEntry` | nothing |
| Use cases | `TransactionLog` | `TransactionEntry` |
| Interface adapters | CSV parser | `TransactionEntry` |
| Interface adapters | JSON serializer | `TransactionEntry` |
| Interface adapters | HTTP handlers | `TransactionLog` |
| Frameworks | Axum router, serde | HTTP handlers |

`TransactionEntry` and `TransactionLog` have no knowledge of HTTP,
CSV, or JSON — they are pure domain types.

## Interfaces

### POST /transactions

Loads a CSV transaction log into memory, replacing any previous log.

- **Content-Type:** `text/csv`
- **Body:** CSV text with `;` delimiter and header row
- **Success:** `200 OK`, empty body
- **Error:** `400 Bad Request`, plain-text error description

### GET /transactions

Returns all records from the in-memory log.

- **Response:** `200 OK`, `application/json`, array of objects
- **Empty log:** `200 OK`, empty array `[]`

#### JSON record shape

```json
{
  "date":       "2024-01-15",
  "symbol":     "AAPL",
  "number":     10,
  "price":      "182.50",
  "commission": "1.00",
  "currency":   "USD"
}
```

`price` and `commission` are serialized as strings to preserve decimal
precision without floating-point representation errors.

## Introduced data structures

```rust
// Entities layer — src/domain/transaction.rs
pub struct TransactionEntry {
    pub date:       NaiveDate,
    pub symbol:     String,
    pub number:     i64,
    pub price:      Decimal,
    pub commission: Decimal,
    pub currency:   String,
}

// Use cases layer — src/domain/transaction_log.rs
pub struct TransactionLog {
    entries: Vec<TransactionEntry>,
}

impl TransactionLog {
    pub fn new(entries: Vec<TransactionEntry>) -> Self;
    pub fn iter(&self) -> impl Iterator<Item = &TransactionEntry>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

`TransactionLog` exposes no storage implementation details. Access is
exclusively through the iterator interface, per project style rules.

## Feature details

### Story 1 — POST /transactions

**Technical decisions:**

- CSV parsed with the `csv` crate, delimiter set to `b';'`.
- `serde` deserializes each row into `TransactionEntry` directly.
  The `#[serde(rename = "...")]` attribute maps CSV header names to
  struct fields where needed.
- `rust_decimal` crate for `Decimal` fields.
- `chrono` with `NaiveDate` for date parsing; format enforced as
  `%Y-%m-%d` (ISO 8601 basic date).
- Each POST **replaces** the entire in-memory log. The acceptance
  criterion "GET returns exactly the same data as in CSV" implies a
  single authoritative log, not an append model.
- Shared state: `Arc<RwLock<Option<TransactionLog>>>` in Axum state.
  Write lock acquired only during POST handler.

**Error handling:**

- Any deserialization error (wrong delimiter, missing column, invalid
  decimal, unparseable date) → log the library error message verbatim
  via `tracing::error!`, return `400`.
- The write lock is only updated after successful full parse; a failed
  POST leaves the previous log intact.

**Test cases:**

| # | Input | Expected |
| --- | --- | --- |
| 1 | Valid CSV, correct headers | 200, log replaced |
| 2 | Valid CSV, single row | 200, log replaced |
| 3 | Empty CSV (header only) | 200, log is empty |
| 4 | Wrong delimiter (`,`) | 400, error logged |
| 5 | Missing column | 400, error logged |
| 6 | Invalid decimal in price | 400, error logged |
| 7 | Invalid date format | 400, error logged |
| 8 | Empty body | 400, error logged |

### Story 2 — GET /transactions

**Technical decisions:**

- Handler iterates `TransactionLog` via `.iter()`, maps each entry to
  a JSON-serializable DTO.
- `Decimal` serialized as `String` (via `serde_with` or custom
  serializer) to avoid float precision loss.
- `NaiveDate` serialized as `String` in `%Y-%m-%d` format, matching
  the CSV input — satisfying the "exactly the same data" criterion.
- No pagination — all records returned. Acceptable given the
  "no external storage" scope (log is bounded by what was POSTed).

**Test cases:**

| # | Precondition | Expected |
| --- | --- | --- |
| 1 | No POST yet | 200, `[]` |
| 2 | POST valid CSV | 200, all rows as JSON |
| 3 | POST then POST again | 200, only last CSV |
| 4 | Date/decimal match CSV values exactly | 200, fields match |

## NFRs

### Observability

- All incoming requests logged at `DEBUG` level (method, path).
- All recoverable errors logged with `tracing::error!`, preserving
  the full library error message (e.g., CSV parse error context).
- Service startup logs the resolved listen address at `INFO` level.
- Logging behavior controlled by `RUST_LOG` environment variable.

### Configuration

- Listen address configured via `PFMAN_LISTEN_ADDR` environment
  variable. Default: `127.0.0.1:8080`.
- Parsed at startup; invalid address format → log error and exit.

### Security

- No authentication required (MVP scope).
- CSV body size: no explicit limit defined — out of scope per SPEC.
  Axum's default body limit applies.

### Code quality

- All code passes `cargo clippy --pedantic`.
- Unit tests cover all non-top-level functions.
- HTTP endpoints covered by `.hurl` test files.
