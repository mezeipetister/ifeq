# IF_EQ

```rust
extern crate ifeq;
use ifeq::*;

assert_eq!(9.is(9).then_print("nine"), "nine");
assert_eq!(9.if_eq(9).then_print("nine"), "nine");
assert_eq!(9.if_eq(9).then(|| "nine"), "nine");
```