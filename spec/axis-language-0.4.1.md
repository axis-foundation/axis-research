# **Axis Language Specification — v0.4.1 (Draft)**

*A minimal logic language co-designed by a human and an AI.*

Axis is an experiment in designing a small, deterministic programming language where the structure is guided jointly by human semantics and AI-generated patterns.

This document defines the **minimal core** of Axis as it exists in v0.4.1.

The goal is clarity, simplicity, and a semantics that is easy to reason about—both for humans and for AI systems.

This is not a full language ecosystem. It is a conceptual core.

---

# 1. **Design Principles**

Axis v0.4.1 follows a few simple rules:

### **1. AI-First Perspective**

The structure was shaped in collaboration with an AI partner.
Human clarity matters, but AI reasoning simplicity takes precedence.

### **2. Deterministic**

Given the same inputs and foreign function behaviors, evaluation produces the same result.

### **3. Minimal**

The core is intentionally small:

* immutable values
* rebind-only variables
* no loops
* no mutation of structures
* no implicit conversions
* no global side-effects baked into the language

### **4. Explicit**

All types are explicit.
Functions, values, async computations, and operations must be written without ambiguity.

### **5. Compositional**

Axis avoids special cases.
Records, tuples, arrays, enums, and functions all follow consistent rules.

### **6. Loopless**

Iteration is expressed through:

* recursion
* combinators (`map`, `flat_map`, `filter`, `fold`, `range`)
* comprehensions, which desugar to the above

---

# 2. **Lexical Structure**

### **Identifiers**

```
identifier = letter { letter | digit | "_" }
```

Case-sensitive ASCII.

### **Literals**

* `Int`: `0`, `42`
* `Float`: `3.14`
* `Bool`: `true`, `false`
* `String`: `"..."` (ASCII for now)
* `Char`: `'c'`
* `Unit`: `()`

### **Comments**

* `// line comment`
* `/* block comment */`

---

# 3. **Types**

Axis includes:

### **3.1 Primitive Types**

```
Int
Float
Bool
String
Char
Unit
Task<T>
```

### **3.2 Composite Types**

#### **Tuples**

```
(Int, String)
```

Parentheses without commas are grouping.

#### **Arrays**

```
[Int]
```

Immutable.

#### **Records**

```
type User {
    id: Int,
    name: String,
}
```

#### **Enums**

```
enum Option<T> {
    None,
    Some(T),
}
```

Variants are namespaced:

```
Option::None
Option::Some(5)
```

#### **Type Aliases**

```
type IntPair = (Int, Int);
```

---

# 4. **Modules**

Minimal namespacing:

```
module my.app.core

import util.math
import util.strings as s
```

All top-level definitions (`fn`, `async fn`, `type`, `enum`, etc.) are exported.

Physical file layout is intentionally unspecified.

---

# 5. **Bindings & Rebinding**

Axis separates **names** from **values**.

Names can be rebound, but values are immutable.

```axis
let x: Int = 1;
x = x + 1;       // x now holds 2
```

Rules:

* `let x: T = expr;` introduces a binding
* `x = expr;` rebinds an existing name
* structures themselves cannot be mutated:

  * no `arr[i] = v`
  * no `user.name = "X"`

---

# 6. **Functions, Lambdas, Closures**

### **6.1 Named Functions**

```axis
fn add(a: Int, b: Int) -> Int {
    a + b
}
```

The last expression is the return value.

### **6.2 Lambdas**

```axis
let inc: fn(Int) -> Int = |x| x + 1;
```

Parameters may omit type annotations if context suffices.

### **6.3 Closures**

Lambdas capture their environment by value at creation.

---

# 7. **Expressions & Control Flow**

### **Blocks**

```axis
{
    let a: Int = 10;
    a + 5
}
```

A block evaluates to the final expression.

### **If**

```axis
let v =
    if cond { 10 }
    else { 20 };
```

Both branches must have the same type.

### **Match**

```axis
match opt {
    Option::None     => 0,
    Option::Some(x)  => x,
}
```

Must be exhaustive for enums.

---

# 8. **Async & Tasks**

Axis includes a minimal async abstraction.

### **8.1 Async Functions**

```axis
async fn fetch(id: Int) -> String {
    // ...
}
```

An `async fn` returns a `Task<T>`.

### **8.2 Spawn**

```axis
let t: Task<Int> = spawn async { compute() };
```

### **8.3 Await**

```axis
let result: Int = await t;
```

Execution strategy is intentionally unspecified.
The host environment determines how tasks progress and complete.

---

# 9. **Foreign Modules**

Foreign functions allow interaction with the host:

```axis
foreign module sys.io {
    fn read_line() -> String;
    fn print_line(msg: String) -> Unit;
}
```

They behave according to the host environment and may perform side effects.

---

# 10. **Loopless Iteration**

Iteration uses combinators that conceptually behave like:

```axis
fn map<A, B>(xs: [A], f: fn(A) -> B) -> [B];
fn flat_map<A, B>(xs: [A], f: fn(A) -> [B]) -> [B];
fn filter<A>(xs: [A], f: fn(A) -> Bool) -> [A];
fn fold<A, B>(xs: [A], init: B, f: fn(B, A) -> B) -> B;
fn range(n: Int) -> [Int]; // [0..n-1]
```

Axis also provides comprehension syntax:

```axis
let ys =
  [ x * 2
    for x in xs
    if x > 3 ];
```

Comprehensions are **pure sugar**; they desugar to `map`/`flat_map`/`filter`.

---

# 11. **Grammar Reference (Informal)**

A compact overview of core syntax.

### **11.1 Top-Level**

```
program       ::= module_decl? import_decl* top_level_decl*
top_level_decl ::= function_decl
                 | async_function_decl
                 | type_decl
                 | type_alias
                 | enum_decl
                 | foreign_module_decl
```

### **11.2 Functions**

```
fn name (param_list?) -> type block
async fn name (param_list?) -> type block
```

### **11.3 Expressions**

```
expression ::= literal
             | identifier
             | function_call
             | record_literal
             | enum_literal
             | tuple_literal
             | array_literal
             | if_expression
             | match_expression
             | lambda
             | block
             | spawn_expression
             | await_expression
```

### **11.4 Patterns**

```
pattern ::= identifier
          | "_"
          | "(" pattern (, pattern)* ")"
          | Type { field_list }
          | Type::Variant(pattern?)
```

This grammar is intentionally compact.
Full formalization is left to future work.

---

# 12. **Examples**

### Records

```axis
type User { id: Int, name: String }

fn main() -> String {
    let u = User { id: 1, name: "A" };
    u.name
}
```

### Enums & Match

```axis
enum E { A, B(Int) }

fn main() -> Int {
    let v = E::B(5);
    match v {
        E::A    => 1,
        E::B(x) => x,
    }
}
```

### Rebinding

```axis
fn main() -> Int {
    let x = 0;
    x = x + 1;
    x = x + 2;
    x
}
```

### Loopless

```axis
fn main() -> Int {
    let xs = [1, 2, 3];
    let doubled = map(xs, |n| n * 2);
    fold(doubled, 0, |acc, n| acc + n)
}
```

### Async

```axis
async fn get_value() -> Int {
    5
}

fn main() -> Task<Int> {
    spawn async {
        let v = await get_value();
        v + 1
    }
}
```

---

# **Summary**

Axis v0.4.1 defines a minimal programming language built through a human–AI collaborative design process, emphasizing:

* determinism
* explicitness
* immutability
* rebinding instead of mutation
* loopless iteration
* clean async semantics
* simple, compositional constructs
