# Structures in Ziz

In Ziz, a structure is a mathematical structure, consisting of exactly one set, equipped with a number of operations on which identities are defined. The general syntax for a structure definition is:

```rust
struct StructureName<OverStruture(...)> (
    SetIdentifier;
    Element and operation definitions,...
) {
    identities...
}
```

For example, a basic structure with one commutative operation defined on it would be defined by:

```rust
struct BasicStructure(
    S;
    * : S * S -> S, 
) {
    \-/ a, b (- S {
        a * b = b * a
    }
}
```

The statement above only defines the structure `BasicStructure`. To create an instance of the structure we must use a `let-be` statement:

```rust
let (S; *) be BasicStructure
```

This expression creates a `Set` and a `BinaryOperation`, and binds them into the global scope as `S` and `*` respectively. It also attaches to these variables all the identities associated with `BasicStructure`, so that they can be used in symbolic computation.


## Structure Dependencies

Some mathematical structures must be defined *over* one or more other structures - e.g. the Vector Field which must be defined over some suitable Field.

Ziz provides a mechanism for defining structures which depend on other structures using the **over structures** array.