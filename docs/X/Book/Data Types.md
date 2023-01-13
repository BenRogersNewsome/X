---
title: Data Types in X
---

In X, data types take a slightly different role than in other languages. X defines 3 data types:
- `Element`
- `Set`
- `Operation`

# Elements

`Element`s are the smallest unit of an X program, and simply represent *some* mathematical entity. They can be created using the `let` keyword:

```rust
let a, b
```

Elements should be thought of as atomic variables: They contain no data and simply refer to themselves.

In the example above, `a` and `b` are referred to as **anonymous** elements as we have made no assertions about what `a` and `b` *are*.

# Sets

A `Set` is a collection of elements. They can also be created using the `let` keyword, with `Set` literals being declared using set-builder notation. We can create a `Set` containing the elements `a` and `b` from before, as:

```rust
let S be {a, b}
```

We can create the empty `Set` by using empty braces:

```rust
let O be {}
```

## Sets in Depth

We might want to create a `Set` which contain an unknown number of elements, which contains a mixture of known and unknown elements. To achieve this we use the spread syntax `...`. For example, to create a set which contains `a`, `b` *and* an arbitrary number of additional elements, we would write:

```python
# `S` contains `a`, `b` and some non-zero number of additional elements
let S be {a, b, ...}
```

In X, `S` is referred to as **incomplete** as there is incomplete information as to what `S` contains. We can create a fully incomplete `Set`, referred to as an **anonymous** `Set`, by writing:

```python
# `S` contains some non-zero number of elements
let S be {...}
```

# Operations

`Operation`s are user-defined symbols which act on elements of sets in X. X supports binary and unary operations. They can be defined using the `def` keyword:

```python
# `*` is a binary operation which acts between two elements of `S` yielding another element of `S`.
def * : S * S -> S

# `!` is a unary operation which acts on one element of `S` yielding another element of `S`.
def ! : S -> S
```

## Assertions

Simply defining an operation gives X no information about how the operation behaves, and hence no information with which to do symbolic computation. To define properties on an operation, we can use an assertion block:

```rust
|- \-/ a, b (- S {
    a * b = b * a
}
```

The assert block serves two functions in X:
1) Define the properties of operations and sets
2) Check that the assertions made are self-consistent with previous logic.

This second property of assertions is useful for checking mathematical logic. For example, say we create two sets:

```rust
let s
let S be {s}
let P be {}
```

The assertion `|- s (- P` will fail since `P` is an empty set. On the other hand, if we defined `S` and `P` as,

```rust
let S be {s}
let P be {...}
```

Then the assertion `|- s (- P` would pass, with X now using the fact that `s (- P` in all future computation.

This property of the assert block can sometimes be undesirable if you simply want to ask X a question without making a concrete assertion. For this reason, you can disable the first property of the assert block, and simply do a mathematical consistency check, by putting a `?` at the end of the assert statement:

```rust
|- s (- P ?
```

This assertion will now fail since, although there is no reason `s` *couldn't* be a member of `P`, `s` has not been explicitly declared as a member of `P`.

This syntax also works for whole assert blocks:

```python
|- \-/ a, b, c (- S {
    # ...
} ?
```

### Assertions on Sets

Assertions can also be used to add structure to sets. For example suppose we defined a set and an operation as:

```python
let 1  # Define an element called `1`
let S be {1, ...}  # Create a set with `1` in it
def * : S * S -> S  # Define a binary operation in S
```

We could use an assert block to assert that `1` is the identity in `S`:

```python
|- \-/ a (- S {
    a * 1 = a
}
```

We could also assert that the inverse of `*` always exists in `S`:

```python
|- \-/ a (- S -] b (- S {
    a * b = 1
}
```



## Operator Overloading

Consider the X code:

```python
let A be {...}
let B be {...}

def * : A * A -> A 
def * : B * B -> B
def * : A * B -> B
```

At first glance the code looks invalid as we have defined the operator `*` 3 times! However, the code is actually completely valid, due to the way that operators are scoped in X.

In X, operators are not only identified by their symbol, but also by the set elements they are applied to. In each of the 3 definitions above `*` acts between elements of differing pairs of sets. For this reason, X will always be able to uniquely figure out which definition to use:

```python
let a (- A
let b (- B

a * b  # Uses definition 3
a * a  # Uses definition 1
```

This is known as operator overloading and allows you to use the same symbol to represent operations between different elements, just as you would in regular written math.

The same is true of operation constraints:

```python
\-/ a_1, a_2 (- A, b (- B {
    a_1 * a_2 = - (a_2 * a_1)  # Applies to definition 1
    b * b = b  # Applies to definition 2
    a_1 * b = b * a_1  # Applies to definition 3
}
```

### Overloading Exceptions

Some types of operator overloading are not allowed in X, such as,

```rust
def * : A * A -> A
def * : A * A -> B
```

In this situation, given any two set elements in `A`, X has no way to work out which operation definition to use, and so X will raise a runtime error.
