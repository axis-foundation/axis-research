## 1. Axis Source

*A minimal deterministic logic syntax*

Axis source code is designed to have:

* explicit types
* immutable values
* recursion over looping constructs
* no hidden state or side effects

This creates a predictable environment for reasoning, clean transformation, and formal semantics.

---

## 2. MIR (Minimal Intermediate Representation)

*A provable, SSA-style semantic core*

Axis source is transformed into MIR, a simplified internal representation with:

* well-typed data flow graphs
* explicit control flow
* no mutation
* no implicit state

MIR is where semantic guarantees are enforced and where future formal reasoning tools can operate.

---

## 3. Execution Backend (LLVM — planned)

*A path to real compiled code*

MIR can eventually target **LLVM** to achieve:

* static compilation
* JIT execution
* multi-architecture support (x86, ARM, WASM, etc.)

This decouples **semantic correctness** from **hardware execution**, enabling Axis to remain small and focused while benefiting from existing compiler infrastructure.

---

## Why MIR exists

Most languages mix semantic reasoning with execution details. Axis separates them cleanly:

* **Source**: human/AI-readable form
* **MIR**: provable logic form
* **Backend**: performance and optimization

This design keeps Axis small, simple, and explicit, while still offering a path to high performance execution.

---

## Current Status

* v0.4.x minimal syntax specification
* prototype MIR design in discussion
* backend design in exploration phase

Axis is early, experimental, and evolving based on community feedback.

---

## Motivation

What if a language didn’t optimize for human ergonomics, but instead optimized for:

* clarity of reasoning
* explicit semantics
* AI generation and transformation of logic

Axis is an experiment along that line of thought.