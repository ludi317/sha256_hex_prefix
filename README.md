
Comparison of Rust and Go implementations of finding sentences that describe the prefix of their SHA256 hex strings.

Inspired from https://news.ycombinator.com/item?id=37465086

Go:

```
$ go build . && time ./sha256
The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine.
182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0
./sha256  31.93s user 0.41s system 99% cpu 32.434 total
```

Rust:
```
$ cargo build --release && time ./sha256_hex_prefix 7
    Finished release [optimized] target(s) in 0.08s
The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine.
182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0
./sha256_hex_prefix 7  26.01s user 0.26s system 99% cpu 26.368 total
```

* Rust is 19% faster than Go.
* Rust program accepts the length of the hex string prefix as an argument, whereas in Go it is hardcoded to length 7. 
Zero-cost abstraction in Rust allows for this customization without sacrificing performance.
