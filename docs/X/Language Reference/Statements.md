
# Top Level Items

## Let

```asn
let_stmt ::= "let" ( 
    set_creation |
    set_element_creation |
    struct_instantiation
)

set_creation ::= "(" identifier ")" "be" identifier

set_element_creation ::= identifier "(-" identifier

struct_instantiation ::= struct_signature "be" identifier["<" argument_list ">"]
```

## Def

```asn
"def" operation_definition
```

## Math Block

```
logic_block ::= logical_expr "{"
    math_expr*
"}"
```

## Struct

```asn
struct_def ::=
"struct" identifier "("
    identifier ";"
    (
        (op_def | identifier) ","
    )*
")" "{"
    logic_block*
"}"

```

The `struct` statement is used to define a mathematical structure.

### Examples

To define a **Group** in ziz:

```rust
    struct Group (
        G,
        * : G * G -> G,
        inv : G -> G,
        I,
    ) {
        \-/ a (- G {
            a * I = a
            inv(a) * a = I
        }

        \-/ a, b, c (- G {
            (a * b) * c = a * (b * c)
        }
    }
```

# Other Items


```asn
argument_list ::= identifier ("," Identifier)* [","]

logical_expr ::= (forall_expr | therex_expr)+

forall_expr ::= ("\-/"|"forall") identifier ("," identifier)* ("(-"|"in") identifier

therex_expr ::= ("-]"|"therex") identifier ("," identifier)* ("(-"|"in") identifier

math_expr ::= binary_expr

binary_expr ::= unary_expr binary_operator unary_expr | unary_expr

unary_expr ::= identifier | unary_operator identifier

op_def ::= symbol ":" identifier [symbol identifier] -> identifier

struct_signature ::= "(" identifier ";" argument_list ")"
```