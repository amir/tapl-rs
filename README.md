```
$ rustup run nightly cargo run
> iszero (pred (succ zero))
true
> if (iszero (pred (succ zero))) then (succ zero) else zero
0
> pred (succ (succ (succ zero)))
2
```
