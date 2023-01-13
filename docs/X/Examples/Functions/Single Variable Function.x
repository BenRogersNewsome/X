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

let (F; +, *, 0, 1) be Field

def * : F * F -> F

def f : (x: F) -> x*x

# Or

def f(x: F) = x*x

# Or ?

let x (- f
def f(x) = x*x

let x in F