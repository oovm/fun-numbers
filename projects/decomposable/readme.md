A super prime is a prime number that, when any of its digits is deleted, the remaining number is still prime.

eg:
- 1 -> 19 -> 199 -> 1999 -> 13999 -> ...
- 2 -> 29 -> 269 -> 2969 -> 25969 -> ...

So we called 25969 a super prime.

## Examples

```
# use super_prime::{BigUint, super_prime};
let start = BigUint::from(2usize);
for n in super_prime(&start, 100).into_iter().rev() {
    println!("{}", n);
}
```