struct Field (
    F;
    + : F + F -> F,
    * : F * F -> F,
    0,
    1,
) {
    |- \-/ a, b, c (- F {
        a + b = b + a
        a + 0 = a
        (a + b) + c = a + (b + c)

        a * 1 = a
        a * b = b * a
        (a * b) * c = a * (b * c)
        a * (b + c) = (a*b) + (a*c)
    }
}

struct VectorSpace<Field(F; +, *, 0, 1)> (
    V;
    + : V + V -> V,
    * : F * V -> V,
) { 
    |- \-/ v, u, w (- V, a (- F {
        v + u = u + v
        v + u = u + v
        (v + u) + w = v + (u + w)
        a * (u + v) = (a*u) + (a*v)
    }
}

let (F; +, *, 0, 1) be Field
let (V; +) be VectorSpace<F>

let a, b, in F