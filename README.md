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
$ rustup run nightly cargo run --bin tyarith
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

## simplebool
```
$ rustup run nightly cargo run --bin simplebool
> (lambda x:Bool. x)
(lambda x:Bool. x) : Bool->Bool
> (lambda x:Bool->Bool. x)
(lambda x:Bool->Bool. x) : Bool->Bool->Bool->Bool
> ((lambda x:Bool->Bool. x) (lambda y:Bool. y))
(lambda y:Bool. y) : Bool->Bool
```

## fullsimple
```
$ rustup run nightly cargo run --bin fullsimple
> T=Nat;
T
> (lambda i:T. succ i) 12
13: Nat
> (lambda x:Bool->Bool. if x false then true else false)(lambda x:Bool. if x then false else true)
true: Bool
> (lambda x:Bool. x) 11
ContextError(ParameterTypeMismatch)
```

Inspired by https://github.com/hayatoito/tapl-in-rust
