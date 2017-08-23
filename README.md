## arith
```
$ rustup run nightly cargo run --bin arith
> iszero (pred (succ zero))
true
> if (iszero (pred (succ zero))) then (succ zero) else zero
0
> pred (succ (succ (succ zero)))
2
```

## fulluntyped
```
$ rustup run nightly cargo run --bin fulluntyped
> (lambda a. (lambda a. (a a)))
(lambda a. (lambda a'. (a' a')))
```
