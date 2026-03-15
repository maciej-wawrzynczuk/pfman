# Transaction log services

## Stories

1. Create a web service that allows loading a CSV transaction log. POST method. Just text/csv.
2. Create a web service which returns the log as JSON. Display all records. (GET)

## Requirements

- Design a clear interface for the transaction log. Implement a struct for the entry. Implement a collection accessible via iterator.
- CSV fields (delimiter is `;`)
  - date: Format in csv: `yyyy-mm-dd`
  - symbol: string
  - number: int
  - price: decimal
  - commission: decimal
  - currency: string

## Out of scope

No external storage needed.

## Acceptance

The GET endpoint should return exactly the same data as in CSV.

All format errors should log exact messages from deserialization. And return 400.

## Implementation notes.

- Use basic ISO 8601 date.
- For transation log implementation:
  - you can expose entry fields,
  - Do not expose storage algorhitm details.
  - Expose iterator for sequential access.