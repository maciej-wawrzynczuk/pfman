# Setup: reset to empty log
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
```
HTTP 200


# TC1: No data posted (empty log) - 200, empty array
GET http://127.0.0.1:8080/transactions
HTTP 200
[Asserts]
jsonpath "$" count == 0


# Setup: POST valid CSV
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;182.50;1.00;USD
2024-01-16;MSFT;5;370.00;0.50;EUR
```
HTTP 200


# TC2: GET returns all rows as JSON
GET http://127.0.0.1:8080/transactions
HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$[0].symbol" == "AAPL"
jsonpath "$[1].symbol" == "MSFT"


# Setup: POST again (second CSV replaces first)
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-03-01;GOOG;3;140.00;0.25;USD
```
HTTP 200


# TC3: GET returns only last CSV
GET http://127.0.0.1:8080/transactions
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].symbol" == "GOOG"


# TC4: Date and decimal fields match CSV values exactly
GET http://127.0.0.1:8080/transactions
HTTP 200
[Asserts]
jsonpath "$[0].date" == "2024-03-01"
jsonpath "$[0].price" == "140.00"
jsonpath "$[0].commission" == "0.25"
