# About this Crate

This is a simple utility for SQL queries to calculate the offset and limit.

# Disclaimer

The author takes no responsibility for any bugs that occur from the use of this crate.

# License

MIT

# Usage

```rust
let LimitOffsetPair { limit, offset } = sql_paginatorr::for_page(3, 10); // 40, 30
```
