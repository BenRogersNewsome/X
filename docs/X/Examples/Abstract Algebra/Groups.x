
struct Group (
    G;
    +,
    0
) {

    |- \-/ a, b, c (- G {
        (a + b) + c = a + (b + c)
        a + 0 = a
    }

    |- \-/ a (- G -] b {
        a + b = 0
    }

}


# Create a Group with an arbitrary number of elements
let (G; *, 1) be Group{...}


# Create a group with two known elements and close over the rest of the group
let a, b
let (G; *, 1) be Group {a, b, ..}


# Create a group with two known elements, close over the rest of the group and then add an arbitrary number of other elements
let a, b
let (G; *, 1) be Group{a, b, .., ...}
