# TC1: Valid CSV, correct headers - 200, log replaced
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;182.50;1.00;USD
2024-01-16;MSFT;5;370.00;0.50;EUR
```
HTTP 200


# TC2: Valid CSV, single row - 200, log replaced
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;182.50;1.00;USD
```
HTTP 200


# TC3: Empty CSV (header only) - 200, log is empty
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
```
HTTP 200


# TC4: Wrong delimiter (comma) - 400
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date,symbol,number,price,commission,currency
2024-01-15,AAPL,10,182.50,1.00,USD
```
HTTP 400


# TC5: Missing column - 400
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;182.50;1.00
```
HTTP 400


# TC6: Invalid decimal in price - 400
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
2024-01-15;AAPL;10;not-a-number;1.00;USD
```
HTTP 400


# TC7: Invalid date format - 400
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
date;symbol;number;price;commission;currency
15-01-2024;AAPL;10;182.50;1.00;USD
```
HTTP 400


# TC8: Empty body - 400
POST http://127.0.0.1:8080/transactions
Content-Type: text/csv
```
```
HTTP 400
