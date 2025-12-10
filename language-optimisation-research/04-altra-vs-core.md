# **04-ultra-vs-core.md — Axis Core vs Axis-Ultra Comparative Analysis**

## **1. Summary**

Axis Core = small, practical, still expressive.
Axis-Ultra = extremely small, theoretically beautiful, restrictive.

Both are viable.
The goal is to empirically discover which is *optimal*.

---

## **2. Dimensions of Comparison**

| Dimension                 | Axis Core    | Axis-Ultra   | Expected Winner |
| ------------------------- | ------------ | ------------ | --------------- |
| **Semantic size**         | Small        | Very small   | Ultra           |
| **Compiler effort**       | Low          | Very low     | Ultra           |
| **Expressiveness**        | High         | Moderate     | Core            |
| **Proof burden**          | Moderate     | Very low     | Ultra           |
| **AI comprehension**      | To be tested | To be tested | Unknown         |
| **Code verbosity**        | Low          | Higher       | Core            |
| **Termination guarantee** | Optional     | Guaranteed   | Ultra           |
| **Real-world use**        | Easier       | Harder       | Core            |

---

## **3. Risks of Axis-Ultra**

* Might be *too minimal* for real programs
* Might require too much desugaring
* Might produce overly verbose encodings

---

## **4. Potential Advantages**

* Canonical normal form simplifies AI training
* Deterministic evaluation improves predictability
* Pure total semantics ideal for theorem provers
* Minimal rules reduce LLM error footprint

---

## **5. Experiment Plan**

To compare:

1. Implement both cores in Python.
2. Build a desugarer from Surface Axis → Core/Ultra.
3. Generate synthetic programs.
4. Run LLM evaluation scripts.
5. Use Lean/Coq to mechanize preservation/progress.
6. Benchmark real code generation examples.

