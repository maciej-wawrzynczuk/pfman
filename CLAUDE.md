# Rules

Follow the workflow: <https://github.com/maciej-wawrzynczuk/ai-rules/blob/main/WORKFLOW.md>.
It's not complete yet. I actively working on it. Check every time.

## Project techstack and coding style

### Rust

- Axum, tower for webservices.
- logging with the tracing package. Behavior controlled by RUST_LOG variable.
- all code must pass unit tests, cargo check, cargo clippy with pedantic.
- Prefer interfaces over loops when sequentially accessing data structures.

### Testing debug and testability

- Write unit tests everywhere excepting top level functions. Top level functions must contain no testable logic.
- Test web services endpoints with hurl.
- All recoverable errors must be logged. All debug info from libraries must be preserved.

### Orchestration

- Use gnu make. For testing use following targets: 

### Pre commit

- use [prek](https://github.com/j178/prek)
- always check markdown against markdownlint-cli2
