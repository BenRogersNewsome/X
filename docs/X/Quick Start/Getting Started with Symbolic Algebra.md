---
title: Getting Started with Symbolic Algebra
---

One of the most common ways to use `X` is for symbolic computation. The base language doesn't define any basic mathematical entities or operations for performing even basic algebra, however, the standard library contains some useful bindings for setting up an environment for symbolic computations.

To set up an environment for symbolic algebra, first import the `RealNumbers` from the standard library, and then bind them to the digits:

```python
from core.structures import RealNumbers
```

This statement imports the real numbers and binds them to the symbols `0, 1, 2, ...`.

From here we can begin