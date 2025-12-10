# **1. The Core Idea**

> Axis Core represents the meaning of code — not the syntax.

It is a tiny set of concepts that every domain can reduce to.

The core contains:

* **immutable values**
* **records** (key/value structures)
* **lists**
* **pure functions**
* **function application**
* **branching** (`if`)
* **no loops, no mutation, no side effects**

Everything else — loops, patterns, expressions, SQL, CSS, API contracts —
reduces to these primitives.

This is deliberate.

---

# **2. Why the Core Is So Small**

Because the smaller the meaning model is:

* the easier it is for tools and AI to understand
* the easier it is to guarantee correctness
* the easier it is to translate code between languages
* the easier it is to analyse
* the harder it is to get wrong

A tiny semantic universe means fewer places for ambiguity to hide.

---

# **3. Example: SQL → Axis**

```sql
SELECT name FROM users WHERE age > 30
```

Axis:

```axis
{
  select: ["name"],
  from: "users",
  where: fn(row) -> gt(row.age, 30)
}
```

Meaning captured cleanly.

---

# **4. Example: CSS → Axis**

```css
.button { color: blue; }
```

Axis:

```axis
{
  selector: ".button",
  properties: [
    { key: "color", value: "blue" }
  ]
}
```

Structure preserved.
Syntax irrelevant.

---

# **5. Example: Python Function → Axis**

```python
def tax(x): return x * 0.2 if x > 30000 else x * 0.1
```

Axis:

```axis
fn tax(x: Number) -> Number {
  if gt(x, 30000)
  then mul(x, 0.2)
  else mul(x, 0.1)
}
```

The meaning is exactly the same.
The syntax is gone.

---

# **6. Why This Reduces Hallucinations**

AI today must learn:

* Python semantics
* Rust semantics
* JS edge cases
* SQL
* CSS
* every framework under the sun

Axis gives AI **one model to learn** instead of dozens.

This is not magic — it’s simplification.

---

# **7. Why This Helps Tools**

With one semantic model:

* code translation becomes clearer
* refactors become safer
* static analysis becomes easier
* semantic search becomes possible
* reasoning becomes consistent

---

# **8. What Axis Core Does *Not* Do**

It does **not** handle:

* IO
* file access
* GPU work
* concurrency
* number semantics (floats vs ints)
* sockets
* OS interaction

These belong in **bridges**, not the core.

The core describes **meaning**, not behaviour.

---

# **9. Summary**

Axis Core is:

* extremely small
* easy to analyse
* easy to translate into
* easy to map out of
* stable across time
* the shared semantic language for all computation

This is the “heart” of Axis.