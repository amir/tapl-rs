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

## tyarith
```
> if true then zero else false
TypeError("arms of conditional have different types")
> (succ false)
TypeError("argument of succ is not a number")
> if true then zero else (succ zero)
0: Nat
> if true then true else false
true: Bool
> if (succ zero) then true else false
TypeError("guard of conditional not a boolean")
```
