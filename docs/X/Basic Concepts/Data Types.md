# Data Types in X

To keep the language as general as possible, X  doesn't define any primitive data types in the same way that most programming languages do.

X defines two main items of mathematical logic:
- `SetElement`
- `Set` (note that a `Set` can also be a type of `SetElement`)

which can be created using the `let` keyword:

```rust
let (S) be Set
let a, b (- S
```

The code above creates a `Set` with the identifier `S`, and creates two set elements inside of it, `a` and `b`.

## Operations

There are no mathematical operations defined in X by default. Operations are considered as variables in the same way as sets and set elements and can be created using the `def` keyword:

```python
def * : S * S -> S
```

The code above creates an operation labelled `*` which acts between any two elements of the set `S`, returning another element of `S`.

We can define properties of the operation `*`, such as associativity and commutativity, by using the `assert` keyword:

```python
|- \-/ a, b, c (- S {
    (a * b) * c = a * (b * c)
    a * b = b * a
}
```

### Operator Overloading

Consider the X code:

```python
let (A) be Set
let (B) be Set

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

#### Overloading Exceptions

Some types of operator overloading are not allowed in X, such as,

```rust
def * : A * A -> A
def * : A * A -> B
```

In this situation, given any two set elements in `A`, X has no way to work out which operation definition to use, and so X will raise a runtime error.
