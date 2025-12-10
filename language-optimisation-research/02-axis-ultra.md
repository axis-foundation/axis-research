# **02-axis-ultra.md — The Axis-Ultra Core Language (Experimental Candidate)**

> **Status:** Experimental
> **Purpose:** Explore whether a smaller, more canonical core improves AI reasoning, compiler correctness, and formal verification.

---

## **1. Goals**

Axis-Ultra is a sibling to the Axis Core.
It is intentionally more constrained:

* Fewer term constructors
* Fewer type constructors
* Canonical evaluation order (ANF)
* Explicit structural recursion only
* Closed algebra of effects

This design aims to:

* Minimize semantic surface area
* Maximize proof tractability
* Improve AI model stability
* Reduce compiler complexity

---

## **2. Syntax**

### **2.1 Types**

```
T ::= 
      A                        # atomic type
    | T -> T                   # function
    | T × T                    # product
    | T + T                    # sum
    | μX. T                    # inductive type (structural recursion only)
```

Axis-Ultra removes:

* General recursion
* Implicit effects
* Typeclass-like constructs
* Unrestricted higher-order polymorphism (kept optional)

---

## **3. Terms**

Axis-Ultra has **only 3 primitive term constructors**:

```
e ::= 
      x                         # variable
    | λx:T. e                   # lambda
    | e1 e2                     # application
    | case e of L x -> e1 
                 | R x -> e2    # sum elimination only
```

Everything else (let-binding, pairs, injections, pattern binding) is sugar defined in the desugaring rules.

---

## **4. A-Normal Form Discipline**

Axis-Ultra *requires* ANF; no nested applications or case scrutinees.

```
v ::= x | λx:T. e
ae ::= v | v v

e ::= ae
    | let x = ae in e
    | case x of L y -> e1 | R y -> e2
```

Benefits:

* Deterministic evaluation
* Easier optimization
* LLMs get simpler patterns
* Proof systems get simple structural induction

---

## **5. Structural Recursion Only**

General recursion is not allowed.
Instead, recursion must be explicit and tied to the structure of an inductive type.

```
rec list_case(x) { 
    Nil -> e1
    Cons(h, t) -> e2
}
```

Desugars into a fixed, well-founded eliminator for the type.

This guarantees:

* Termination
* Total semantics
* Stronger reasoning properties

---

## **6. Effects**

Effects are modeled *externally* using a fixed carrier algebra:

```
Eff ::= IO | State[S] | Time | Rand
```

Terms remain pure; effects are handled via explicit effect bindings in the surface layer.

This keeps the core mathematically clean while still supporting practical programming.

---

## **7. Small-Step Semantics (Sketch)**

Deterministic evaluation:

```
(λx:T. e) v  →  e[x := v]
case (L v) of ... → e1[x := v]
case (R v) of ... → e2[x := v]
let x = v in e → e[x := v]
```

No ambiguous evaluation order.
No concurrency in the core.
No side effects inside the pure core.

---

## **8. Desugaring Rules**

Examples:

### **Pairs**

```
(a, b)
```

⇒

```
L a + R b        # encoded using sums
```

### **Let-bindings**

```
let x = e1 in e2
```

⇒ *ANF enforced during compilation*

### **Match on lists**

```
match xs with
  [] -> a
  h::t -> b
```

⇒

```
case xs of
  L _ -> a
  R p -> let h = fst p in
         let t = snd p in b
```

---

## **9. Why This Core?**

Axis-Ultra explores the smallest core that:

* Is still typed
* Still supports structured data
* Still supports proofs
* Still maps cleanly to LLVM / Rust / Python
* Still understandable to LLMs

It is not intended as the final design — only as an experimental minimal baseline.

