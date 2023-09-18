### About

Comparison of Rust and Go implementations of finding sentences that describe the first 7 hexadecimals of their SHA256 digests.

Inspired from https://news.ycombinator.com/item?id=37465086
#### Go

```
$ ./sha256_go 
The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine.
182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0
Go time elapsed is: 30.9s
```

#### Rust
```
$ ./sha256_hex_prefix  
The SHA256 for this sentence begins with: one, eight, two, a, seven, c and nine.
182a7c930b0e5227ff8d24b5f4500ff2fa3ee1a57bd35e52d98c6e24c2749ae0
Rust time elapsed is: 33.8 seconds
```

* Rust implementation is about as fast as Go. They are dominated by the time it takes to compute SHA256 digests. Each one imports a cryptographic library with assembly implementations for performance. 
* The Rust program is more flexible, since it accepts the length of the hex string prefix as an argument. Zero-cost abstraction in Rust _should_ allow this configurability without sacrificing performance. 
* The Go program has 7 nested for-loops, each one iterating over all hexadecimals, fixing the prefix length to 7. 


### Bonus Round
This sentence describes the first 8 hexadecimals of its SHA256:
```
The SHA256 for this sentence begins with: c, seven, e, seven, c, two, eight and three.
c7e7c28309457ce759bf7850dede9af83ac9747489eb9cb958227f37bff8aa37
```