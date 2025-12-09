# Axis Core (v0.MIN)

Axis Core is the **minimal, deterministic kernel calculus** for Axis.

Everything else in the language (records, enums, modules, async, tasks, foreign calls, etc.) is defined as **layers** or **desugarings** on top of this core.

Axis Core has:

* immutable values
* rebinding via shadowing
* functions and tuples
* blocks and `if`
* call-by-value evaluation
* no effects, no concurrency, no IO

---

## 1. Lexical & Surface Conventions

### 1.1 Identifiers

```text
identifier = letter { letter | digit | "_" }
```

Case-sensitive ASCII.

### 1.2 Literals

Axis Core has three primitive literal categories:

* `Int`: `0`, `1`, `-5`, `42`, ...
* `Bool`: `true`, `false`
* `Unit`: `()`

Comments and modules are outside the core.

---

## 2. Types

Axis Core types:

```text
T ::= Int
    | Bool
    | Unit
    | (T1, ..., Tn)           // n ≥ 2, tuple types
    | T -> T                  // function type
```

Notes:

* Multi-argument functions in the surface syntax are **sugar** over single-argument functions on tuples.
  Example:

  ```axis
  fn add(a: Int, b: Int) -> Int { a + b }
  ```

  is core-level:

  ```text
  add : (Int, Int) -> Int
  add = λp:(Int,Int). p.1 + p.2
  ```

---

## 3. Terms: Values, Expressions, Blocks

Axis Core distinguishes **values** (fully evaluated) from general expressions.

### 3.1 Values

```text
v ::= n                           // integer literal
    | true | false
    | ()
    | (v1, ..., vn)               // tuple of values
    | λx:T. e                     // lambda (closure)
```

### 3.2 Expressions

```text
e ::= v                           // value
    | x                           // variable
    | λx:T. e                     // abstraction
    | e1 e2                       // application
    | (e1, ..., en)               // tuple
    | e.k                         // tuple projection, 1 ≤ k ≤ n
    | let x:T = e1 in e2          // let-binding
    | if e_cond { e_then } else { e_else }
    | { stmt* e }                 // surface block (desugared)
```

### 3.3 Statements (Surface Sugar Only)

At the surface, blocks allow simple statements:

```text
stmt ::= let x:T = e;             // binding
       | x = e;                   // rebinding
```

Blocks:

```axis
{
  stmt1;
  stmt2;
  ...
  stmtn;
  expr
}
```

are **not** primitive in the core; they are desugared to nested `let` expressions.

---

## 4. Desugaring: Rebinding & Blocks

Axis Core has **no** mutable variables. “Rebinding” is implemented by **shadowing** via `let`.

### 4.1 Rebinding Desugaring

A block:

```axis
{
  let x: T = e0;
  x = e1;
  x = e2;
  e3
}
```

desugars to:

```text
let x:T = e0 in
let x:T = e1 in
let x:T = e2 in
e3
```

In general:

* `let x:T = e;` stays `let x:T = e in ...`
* `x = e;` becomes another `let x:T = e in ...` (with the same `T` as the original binding).

### 4.2 Block Desugaring

Given:

```axis
{
  s1;
  s2;
  ...
  sn;
  e
}
```

desugar sequentially:

1. Turn each `stmt` into a `let` (binding or shadowing).
2. Nest them around `e`:

Example:

```axis
{
  let a: Int = 10;
  let b: Int = a + 1;
  b
}
```

becomes:

```text
let a:Int = 10 in
let b:Int = a + 1 in
b
```

After desugaring, the **core** only sees `let x:T = e1 in e2`, not statements or braces.

---

## 5. Typing

We write `Γ ⊢ e : T` to mean “under type environment Γ, expression e has type T”.
Γ is a finite mapping from identifiers to types.

### 5.1 Primitive Typing

```text
(T-Int)
  Γ ⊢ n : Int

(T-Bool)
  Γ ⊢ true : Bool
  Γ ⊢ false : Bool

(T-Unit)
  Γ ⊢ () : Unit
```

### 5.2 Variables

```text
(T-Var)
  x : T ∈ Γ
  -----------
  Γ ⊢ x : T
```

### 5.3 Tuples

```text
(T-Tuple)
  Γ ⊢ e1 : T1   ...   Γ ⊢ en : Tn
  --------------------------------
  Γ ⊢ (e1, ..., en) : (T1, ..., Tn)
```

### 5.4 Tuple Projection

For 1-based index `k`:

```text
(T-Proj)
  Γ ⊢ e : (T1, ..., Tk, ..., Tn)
  ------------------------------
  Γ ⊢ e.k : Tk
```

### 5.5 Functions

Lambda abstraction:

```text
(T-Abs)
  Γ, x:T1 ⊢ e : T2
  -----------------------
  Γ ⊢ λx:T1. e : T1 -> T2
```

Application (call-by-value typing):

```text
(T-App)
  Γ ⊢ e1 : T1 -> T2    Γ ⊢ e2 : T1
  --------------------------------
  Γ ⊢ e1 e2 : T2
```

### 5.6 Let-Binding

```text
(T-Let)
  Γ ⊢ e1 : T1      Γ, x:T1 ⊢ e2 : T2
  ----------------------------------
  Γ ⊢ let x:T1 = e1 in e2 : T2
```

### 5.7 Conditional

```text
(T-If)
  Γ ⊢ e_cond : Bool
  Γ ⊢ e_then : T
  Γ ⊢ e_else : T
  -----------------------------------------------
  Γ ⊢ if e_cond { e_then } else { e_else } : T
```

Blocks `{ stmt* e }` are typed by first desugaring to nested `let` expressions, then applying the rules above.

---

## 6. Evaluation (Small-Step Semantics)

Axis Core uses **deterministic**, **call-by-value** small-step semantics.

We write `e → e'` for “expression e steps to e'”.

### 6.1 Values Do Not Step

If `e` is a value `v`, there is no `e'` such that `v → e'`.

### 6.2 Function Application

Evaluation order: left-to-right, call-by-value.

```text
(E-App1)
  e1 → e1'
  ----------------
  e1 e2 → e1' e2

(E-App2)
  v1 e2 → v1 e2'
  e2 → e2'
  (where v1 is a value)
```

When both function and argument are values and the function is a lambda:

```text
(E-Beta)
  (λx:T. e) v → e[x := v]
```

(Substitution `e[x := v]` is the usual capture-avoiding substitution.)

### 6.3 Tuples

Left-to-right evaluation of components:

```text
(E-Tuple1)
  e1 → e1'
  ----------------------------
  (e1, e2, ..., en) → (e1', e2, ..., en)

(E-Tuplek)
  ek → ek'
  ------------------------------------------------
  (v1, ..., v(k-1), ek, ek+1, ..., en)
    → (v1, ..., v(k-1), ek', ek+1, ..., en)
```

When all components are values, the tuple is a value and does not reduce.

### 6.4 Tuple Projection

Once the subject is a value tuple:

```text
(E-Proj)
  (v1, ..., vk, ..., vn).k → vk
```

### 6.5 Let-Binding

First evaluate the bound expression:

```text
(E-Let1)
  e1 → e1'
  -----------------------------
  let x:T = e1 in e2 → let x:T = e1' in e2
```

When the bound expression is a value:

```text
(E-Let2)
  let x:T = v in e2 → e2[x := v]
```

### 6.6 Conditionals

Evaluate the condition first:

```text
(E-If1)
  e_cond → e_cond'
  --------------------------------------------------
  if e_cond { e_then } else { e_else }
    → if e_cond' { e_then } else { e_else }
```

Then choose the appropriate branch:

```text
(E-IfTrue)
  if true { e_then } else { e_else } → e_then

(E-IfFalse)
  if false { e_then } else { e_else } → e_else
```

### 6.7 Blocks

Blocks `{ stmt* e }` have **no primitive evaluation rule**.
They are first desugared to nested `let` expressions and then evaluated with the rules above.

---

## 7. Out of Scope for Axis Core

The following **are not part of Axis Core** and are defined in higher layers or via desugaring:

* Records, enums, type aliases
* Modules and imports
* Arrays, lists, maps, comprehensions
* `Task<T>`, `async`, `await`, `spawn`
* Foreign functions, IO, effects
* Concurrency, scheduling, networking

These features are either:

* **syntactic sugar** over Axis Core constructs (e.g. multi-arg functions, records as tuples), or
* **definitional extensions** that translate to Axis Core plus runtime-provided operations.

Axis Core remains a **pure, deterministic foundation** suitable for formalization and machine-checked proofs.