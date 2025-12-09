# **Axis Surface Language & Sugar Specification**

### *Version 0.5 — Draft*

*A formally defined syntax layer for humans and AI, lowering deterministically into Axis Core.*

---

# **0. Overview**

Axis separates its semantics into two layers:

1. **Surface Language** — the rich, expressive notation written by humans and AI.
2. **Axis Core** — the minimal calculus of:

   * functions
   * tuples
   * let-binding
   * conditionals
   * deterministic evaluation

This document defines:

* The **syntax** of the Axis surface language
* The **static semantics** (typing & binding rules)
* The **sugar constructs**
* The **formal desugaring rules** that expand surface constructs into core terms

The surface layer contains *no intrinsic semantics*:
it exists solely through its deterministic lowering to Axis Core.

---

# **1. Lexical Structure**

### 1.1 Identifiers

```
identifier = letter { letter | digit | "_" }
```

Case-sensitive. No keywords allowed as identifiers.

### 1.2 Literals

```
Int    ::= 0 | [1-9][0-9]*
Bool   ::= "true" | "false"
String ::= "..."  (UTF-8 allowed)
Char   ::= 'c'
Unit   ::= ()
Float  ::= [digit]+ "." [digit]+    (foreign)
```

Floats, strings, and chars have *foreign semantics* (runtime-defined).

### 1.3 Comments

```
// line comment
/* block comment */
```

---

# **2. Types**

The surface type system includes:

### 2.1 Primitive Types

```
Int
Bool
String
Char
Float
Unit
```

### 2.2 Composite Types

* **Tuples**
  `(T1, T2, …, Tn)`
* **Arrays**
  `[T]`
* **Records**

  ```
  type User { id: Int, name: String }
  ```
* **Enums (Sum Types)**

  ```
  enum Option<T> { None, Some(T) }
  ```
* **Function Types**
  `fn(A, B) -> C`
* **Async Types**
  `Task<T>` sugar for foreign `Unit -> T`

---

# **3. Expressions (Surface)**

Surface expressions include all constructs that AI and humans write:

```
expr ::=
    literal
  | identifier
  | fn_call
  | lambda
  | record_literal
  | field_access
  | enum_construct
  | tuple_literal
  | array_literal
  | if_expr
  | match_expr
  | block
  | let_binding
  | async_block
  | await_expr
  | spawn_expr
  | comprehension
```

The remainder of the document specifies how each of these is lowered.

---

# **4. Modules**

```
module my.app.core

import util.math
import util.string as s
```

No semantics beyond namespacing and scope.

---

# **5. Statements & Bindings**

Surface-level:

```
let x: T = expr;
x = expr;
```

Lowering rules given later.

Bindings are lexically scoped.

---

# **6. Surface Functions**

### 6.1 Function Declaration

```
fn name(param1: T1, param2: T2, ...) -> R { body }
```

### 6.2 Lambda Abstraction

```
|x| expr
|x, y| expr
```

Sugar only; lowers to single-argument lambdas over tuples.

### 6.3 Return Value

Last expression is implicitly returned.

---

# **7. Records**

### 7.1 Record Types

```
type User { id: Int, name: String }
```

### 7.2 Record Construction

```
User { id: 1, name: "A" }
```

### 7.3 Field Access

```
u.name
```

Records lower to tuples; fields lower to tuple projections.

---

# **8. Enums**

```
enum Option<T> {
    None,
    Some(T),
}
```

Variants are constructed as:

```
Option::Some(5)
Option::None
```

Lowering uses integer tags and payload tuples.

---

# **9. Pattern Matching**

Surface:

```
match expr {
    Option::None      => e1,
    Option::Some(x)   => e2,
}
```

Guards allowed:

```
Some(x) if x > 5 => e3
```

Lowers to nested `if` + tuple projections.

---

# **10. Async & Concurrency**

Surface:

```
async { expr }
await t
spawn async { expr }
```

Lowering uses:

* thunks: `Unit -> T`
* foreign `schedule` function
* direct thunk invocation for `await`

---

# **11. Array & Comprehension Sugar**

```
[1, 2, 3]
[ f(x) for x in xs if x > 0 ]
```

Lowers to list constructors and folds.

---

# **PART II — FORMAL DESUGARING RULES**

Every surface construct lowers deterministically into Axis Core.

Notation:

* `⟦ e ⟧` means *lower e into Core*.
* Core constructs include:

  * variables
  * literals
  * lambda
  * application
  * tuple
  * let
  * if

---

# **12. Functions**

### 12.1 Multi-argument functions

Surface:

```
fn f(a: A, b: B) -> R { body }
```

Lowering:

```
⟦ fn f(a, b) -> R { body } ⟧
    = fn f(p: (A, B)) -> R {
        let a = p.1 in
        let b = p.2 in
        ⟦ body ⟧
    }
```

---

# **13. Lambdas**

### Multi-argument lambda

```
|a, b| expr
```

Lowering:

```
|p| let a = p.1 in let b = p.2 in ⟦expr⟧
```

---

# **14. Records**

Given:

```
type User { id: Int, name: String }
```

Field order: `(id, name)`.

### 14.1 Construction

```
User { id: e1, name: e2 }
```

Lowering:

```
( ⟦e1⟧, ⟦e2⟧ )
```

### 14.2 Field access

```
u.name
```

Lowering:

```
u.2
```

(hard-index determined by declaration order)

---

# **15. Enums**

Example:

```
enum Option<T> { None, Some(T) }
```

Lower representation:

* `None` → `(0, ())`
* `Some(x)` → `(1, x)`

Formal lowering:

```
⟦ Option::None ⟧          = (0, ())
⟦ Option::Some(e) ⟧       = (1, ⟦e⟧)
```

---

# **16. Pattern Matching**

Surface:

```
match r {
    None        => e1,
    Some(x)     => e2,
}
```

Lowering:

```
let scrut = ⟦r⟧ in
if scrut.1 == 0 then
    ⟦e1⟧
else
    let x = scrut.2 in ⟦e2⟧
```

With guards:

```
Some(x) if g => e
```

Lowers to nested ifs:

```
if scrut.1 == TAG then
    let x = scrut.2 in
    if ⟦g⟧ then ⟦e⟧ else NEXT
else
    NEXT
```

---

# **17. Async / Await / Spawn**

### 17.1 Async block

```
async { body }
```

Lowering:

```
|_:Unit| ⟦body⟧
```

### 17.2 Await

```
await t
```

Lowering:

```
t(())
```

### 17.3 Spawn

```
spawn async { body }
```

Lowering:

```
schedule(|_:Unit| ⟦body⟧)
```

Where `schedule` is a foreign primitive.

---

# **18. Arrays and Comprehensions**

### 18.1 Literal Arrays

```
[ e1, e2, e3 ]
```

Lowering (example persistent list encoding):

```
Cons(⟦e1⟧, Cons(⟦e2⟧, Cons(⟦e3⟧, Nil)))
```

### 18.2 Comprehensions

```
[ f(x) for x in xs if g(x) ]
```

Lowering:

```
fold( ⟦xs⟧, Nil,
    |acc, x|
        if ⟦g(x)⟧ then
            Cons( ⟦f(x)⟧, acc )
        else
            acc )
```

(Or reversed, depending on desired array order.)

---

# **19. Let-Binding and Rebinding**

### 19.1 Initial binding

```
let x: T = e;
```

Lowering:

```
let x = ⟦e⟧ in ...
```

### 19.2 Rebinding

```
x = e;
```

Lowering:

```
let x = ⟦e⟧ in ...
```

Axis Core does not support mutation; rebinding lowers to shadowing.

---

# **20. Block Expressions**

```
{
    s1;
    s2;
    e
}
```

Lowering:

```
let _ = ⟦s1⟧ in
let _ = ⟦s2⟧ in
⟦e⟧
```

---

# **21. If-expressions**

```
if cond { e1 } else { e2 }
```

Lowering:

```
if ⟦cond⟧ then ⟦e1⟧ else ⟦e2⟧
```

---

# **22. Full Example: From Surface to Core**

Surface:

```axis
fn main() -> Task<Int> {
    spawn async {
        let xs = [1,2,3];
        let doubled = [ x*2 for x in xs ];
        let total = fold(doubled, 0, |acc,x| acc+x);
        total
    }
}
```

Lowering:

```
fn main(_:Unit) -> (Unit -> Int) {
    schedule(
        |_:Unit|
            let xs = Cons(1, Cons(2, Cons(3, Nil))) in
            let doubled =
                fold(xs, Nil,
                    |acc,x| Cons(x*2, acc)) in
            let total =
                fold(doubled, 0,
                    |acc,x| acc + x) in
            total
    )
}
```

---

# **23. Properties of Desugaring**

We require:

### 1. **Determinism**

The lowering function `⟦ · ⟧` is deterministic.

### 2. **Type Preservation**

If `Γ ⊢ e : T` in surface typing,
then `Γ ⊢ ⟦e⟧ : T'` in core typing
(where T' is the core representation of T).

### 3. **Semantics Preservation**

For every surface expression `e`,
evaluation of `e` yields the same result as evaluation of `⟦e⟧`.

These theorems will be proven in the formal Coq/Lean development.

---

# **24. Conclusion**

This document fully defines:

* the Axis surface syntax,
* its static semantics,
* and all sugar constructs,
* along with explicit formal desugaring mappings
* into the minimal Axis Core calculus.

The combination of:

* a tiny, provable core,
* a fully formalized macro layer,
* and an AI-native surface language

makes Axis the first modern language designed simultaneously for formal verification, human expression, and AI code generation.