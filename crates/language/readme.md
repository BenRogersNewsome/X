# Language Syntax

## Defining Mathematical Structures

```
struct Field (F, +, *):
    + : F + F -> F
    * : F * F -> F
where forall a,b in F, therex 0,1 in F:
    a + b = b + a
    a * b = b * a
    a * 1 = a
    a + 0 = a
    ...

struct VectorSpace (V, +, .) over Field(F):
    + : V + V -> V
    . : V . V -> F
where forall a, b, c in V:
    a + b = b + a
    a + (b + c) = (a + b) + c
    ...

```

## Instantiating Mathematical Structures

```
let (F, *, +) = Field

let (V, +, .) = VectorSpace(F)
```