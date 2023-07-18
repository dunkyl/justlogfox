# justlogfox

Static lifetime singleton logger with gimmicks. Warning: might not be useful.

## Running Tests

Some tests add and remove custom loggers and assume that there are no other logging sources. Parallel testing breaks this assumption, and makes these fail nondeterministically. 

```sh
cargo test -- --test-threads=1
```

`--test-threads=1` does not prevent tests from spawning multiple threads themselves.