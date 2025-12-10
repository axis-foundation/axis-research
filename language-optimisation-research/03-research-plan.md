# **03-research-plan.md — Searching for an AI-Optimal Minimal Core**

## **1. Hypothesis**

There exists a minimal, canonical programming core that:

* AI models can reason about more reliably
* Compilers can target more simply
* Proof systems can mechanize more easily
* Human developers can still understand

Axis-Ultra is one candidate.
Axis Core is another.
Others exist in the design space.

---

## **2. Goals**

* Identify the smallest usable semantics
* Compare alternatives empirically
* Determine AI preference through testing
* Evaluate amenability to formal verification
* Validate practical expressiveness

---

## **3. Search Space**

We test the following candidate cores:

1. **Axis Core** (current baseline)
2. **Axis-Ultra** (hyper-minimal typed λ + sums + products)
3. **Axis-Ultra-ANF** (strict ANF everywhere)
4. **Axis-Combinator** (SKI-style with type witnesses)
5. **Axis-Array** (APL-style primitive operators)

---

## **4. Method**

### **4.1 Compiler Complexity Analysis**

For each core:

* Lines of code in interpreter
* Lines in typechecker
* Number of semantic rules
* Cross-language backend difficulty

### **4.2 AI Evaluation**

Measure LLM performance via:

* Round-trip stability
* Syntax error rate
* Semantic misunderstandings
* Length of generated code
* Ability to maintain invariants
* Ease of desugaring

### **4.3 Formal Verification**

Mechanize each core in:

* Coq
* Lean
* Isabelle

Metrics:

* Number of axioms
* Proof burden for common properties
* Complexity of substitution lemma
* Complexity of preservation + progress proofs

### **4.4 Real-Program Stress Tests**

Benchmarks:

* Folds
* Map/filter
* List reversing
* Simple compiler passes
* JSON serializer
* 2D cellular automata

We measure:

* Verbosity
* Readability
* Maintainability
* Error-proneness

---

## **5. Outputs**

Final report will answer:

* Is Axis-Ultra strictly better for AI?
* Which core is easiest to prove correct?
* Is there a clear Pareto frontier?
