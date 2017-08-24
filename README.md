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
> let x = true in (if x then (lambda x. (lambda x. (x x))) else (lambda y. y))
(lambda x. (lambda x'. (x' x')))
> let x = false in (if x then (lambda x. (lambda x. (x x))) else (lambda y. y))
(lambda y. y)
```
