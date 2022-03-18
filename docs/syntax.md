## Symbols

Symbols are automatically declared the first time they are used. I.e. the expression
```
a = bc
```
declares the three symbols `a`, `b` and `c`.


# General Syntax

```
struct VECTOR_SPACE(V) over FIELD(F){
    + : V + V -> V
    * : V * V -> V
    . : V . V -> F

    id: u * (v + w) = u * v + u * w
        for u, v, w in V
    id: v + u = u + v
        for u, v in V
}
```

```
create FIELD Reals
create VECTOR_SPACE ColumnVectors over Reals

let v, u in ColumnVectors

let
    w(u,v) in ColumnVectors
    where w = u * v

let w = u * v
let a = w . u
```

# Computation

```
where w # Output: 'VECTOR_SPACE: ColumnVectors'

where a # Output: 'FIELD: Reals'

? w == u # Output: 'false'
```

