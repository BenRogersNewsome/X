---
title: Basic Concepts
---

X is a generic symbolic algebra package for applying symbolic algebra to a range of user-defined mathematical structures.

X is very generic, and as such many concepts which come as a standard in many languages - numbers, arithmetic operations, etc - are not defined as standard in X. Instead, X defines a set of low level types and logical operations, on top of which arithmetic in a range of algebras can be constructed.

# A Logical Language

*X* is a logical language, as opposed to a fully computational one, and a distinction must be made between the way one can interact with *X* vs a traditional programming language.

In a computational language, like Python or Matlab, the order of logic might be something like:
- Define some data
- Act on the data with some pre-defined operations
- Examine the result

*X* on the other hand acts on data that is potentially incomplete, and makes logical assertions based on the data available to it. For example, in *X* we might:
- Define some sets with some elements it them
- Create some more sets from the intersection 

# Notation

Like most programming languages, X defines a set of keywords which have special meaning in the language. In X, many of these keywords are common terms in mathematical logic which have corresponding symbols in logic. These symbols are not present on most ordinary keyboards, and so X provides a written version, and a symbolic representation for each of these keywords, which can be used interchangeably.

| Keyword    | Symbolic Representation | Meaning                   | Example             |
| ---------- | :---------------------: | ------------------------- | ------------------- |
| `in`       |    `(-`, "∈" U+2208     | Set membership            | `let a, b (- S`     |
| `contains` |          `-)`,          | Set Containment           | `let (S) -) a, b`   |
| `forall`   |    `\-/`, "∀" U+2200    | True for all items        | `\-/ a, b (- G`     |
| `therex`   |    `-]`, "∃" U+2203     | There exists an item      | `-] 0, 1 (- F`      |
| `assert`   |    `\|-`, "⊦" U+22A6    | Assert expression is true | `\|- a + b = b + a` |

> **Note:**
> It is highly recommended to use the font #insert font# when working with X, which contains ligatures for nicely rendering the various mathematical symbols used in X.