# **Axis Function Registry – Deep Dive**

The function registry is one of the core ideas behind Axis.
It is simple, but the consequences are far-reaching.

This document explains:

* what it is
* why it exists
* how it works
* what problems it solves
* how it fits with the semantic core and bridges
* what it enables over time

---

# **1. What the Function Registry Is**

At its heart:

> The registry is a directory of functions where each function has
> **a stable identity** and **immutable behaviour** once published.

That’s it.
No magic.
No complex indexing system.
Just a rule:

* when a function is added to the registry
* its semantics cannot change afterward

If you want a new version,
you publish **a new function**, not an update to the old one.

This is the opposite of how most software ecosystems behave today.

---

# **2. Why Immutability Matters**

In most languages today:

* libraries change
* functions change
* dependencies break
* semantics drift
* documentation lags
* AI tools guess
* tools can’t rely on behaviour remaining stable

Axis takes a different path:

> A function's meaning should be fixed forever.
> Once defined, it never surprises anyone again.

This stability enables two things:

### **(a) AI and tools can reason consistently**

If a function is called `sort_numbers/1`,
its behaviour is *guaranteed* to always be the same as when it was registered.

No guessing about versions.
No wondering if the semantics shifted.

### **(b) Human code becomes safer to maintain**

Refactors, transformations, and higher-level tooling become predictable.

---

# **3. What Goes in the Registry**

A registry entry has:

* **name or ID**
* **signature** (types of inputs and outputs)
* **implementation** (referentially transparent function)
* **optional metadata**

Example:

```axis
register add_numbers: fn(Number, Number) -> Number
```

or with metadata:

```axis
register sort_numbers
  signature: fn(List<Number>) -> List<Number>
  description: "Stable, ascending sort"
```

---

# **4. What Doesn’t Go in the Registry**

The registry does **not** contain:

* mutable state
* objects
* runtime behaviours
* OS handles
* anything that can change over time

Those belong in **bridges**, not in the pure semantic layer.

---

# **5. Why Functions Must Be Pure**

The semantic core expects functions to behave like mathematical functions:

* same input → same output
* no side effects
* no dependence on external state

Why?

Because this keeps semantics *predictable*, *deterministic*, and *portable*.

If a function depends on runtime state, that state must be **passed in explicitly** through the bridge layer.

---

# **6. How This Helps With Versions**

Today:

```
Library v1.2.0 breaks something in your app → you discover it too late.
```

With Axis:

```
sort_numbers_v1
sort_numbers_v2
sort_numbers_parallel
```

Nothing replaces anything.
Nothing disappears.
Nothing breaks unexpectedly.

Tools can reason with complete confidence.

---

# **7. Why This Solves Dependency Hell (Or Gets Close)**

Dependency hell comes from change.

Axis solves this by removing change from the semantics layer entirely.

* Functions don’t change
* Types don’t change
* Behaviour doesn’t change

Bridges can evolve,
but the core semantics do not.

---

# **8. Long-Term Possibilities This Opens**

*(Not promises — just what becomes possible to explore.)*

* semantic search for functions
* automated composition tools
* stable cross-language APIs
* safer refactoring systems
* code translation with high fidelity
* intelligent dependency systems

---

# **9. Summary**

The function registry:

* anchors meaning
* removes drift
* simplifies reasoning
* enables better tooling
* supports cross-language interoperability
* makes Axis stable over time

This is one of the smallest parts of Axis conceptually,
but one of the most impactful in practice.