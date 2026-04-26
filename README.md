# Portfolio manager

## Quotes source

Let's try with [https://twelvedata.com/].

[EOD doc](https://twelvedata.com/docs/market-data/end-of-day-price)
Example:

```shell
https://api.twelvedata.com/eod?symbol=AAPL&apikey=demo
```

## Cache

[redb](https://docs.rs/redb/latest/redb/)
For files location: [https://docs.rs/directories/latest/directories/]

I'm going to store decimals as u128.
