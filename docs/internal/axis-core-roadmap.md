Note: This roadmap covers only the Axis Core Language (syntax, types, semantics,
evaluation model) and corresponds to Paper 2 of the Axis research program and the
current GitHub issues and milestones.

Additional roadmaps for the registry, bridging architecture, and higher-layer
semantics will be developed separately.

Note on Repository Structure

This repository currently combines the conceptual research (Papers 1–8), the
Axis core language specification, and early exploration of registry and
bridging ideas. As the project matures and contributors join, these domains
will be split into separate repositories under a unified organization (e.g.,
axis-foundation), allowing research, language development, and registry
implementation to evolve independently and in parallel.

For now, a single repository provides coherence and simplifies early-stage
discussion, but the long-term plan is intentionally modular.


# # Axis Roadmap

This roadmap outlines the planned evolution of the Axis language and specification.
It provides contributors and collaborators with visibility into near-term and mid-term goals, without requiring completion of the full language design upfront.

Axis is currently in early development.
The roadmap will evolve as the specification matures.

---

## **📌 Phase 1 — Foundations (v0.5)**

*Simplifying and stabilizing the minimal core.*

### Goals

* Reduce the early language surface
* Clarify essential constructs
* Prepare for first static and dynamic semantics sections

### Related Work (Issues)

* **#1** Exploring Reduced Language Surface to Further Simplify Semantics
* **#2** Define Minimal Axis Core Constructs for v0.5
* **#17** Improve Introductory Spec Formatting
* **#16** Add Simple Axis Examples Folder

### Expected Outcome

A smaller, more principled minimal language ready for deeper semantics.

---

## **📌 Phase 2 — Static Semantics (v0.6)**

*Formally describing how Axis programs are validated.*

### Goals

* Introduce minimal type system
* Define static semantics of expressions
* Establish rules for well-formed Axis programs

### Related Work (Issues)

* **#3** Draft Outline for Minimal Type System (v0.6)
* **#4** Static Semantics for Expressions
* **#15** Define Core Value Categories
* **#6** Define Initial “Law Tests” Structure

### Expected Outcome

A stable static semantics foundation that supports deterministic evaluation later.

---

## **📌 Phase 3 — Dynamic Semantics (v0.7)**

*Defining how Axis programs evaluate deterministically.*

### Goals

* Create formal evaluation rules
* Clarify reduction steps
* Define evaluation-time value behavior

### Related Work (Issues)

* **#5** Outline for Dynamic Evaluation Semantics (v0.7)
* **#14** Draft Minimal Deterministic Memory Model
* **#12** Implement Deterministic Evaluation for Expression Subset
* **#13** Error Reporting Strategy for Interpreter

### Expected Outcome

A complete picture of how Axis executes logic, suitable for interpreters and proof work.

---

Understood — you want **Phase 4 — Grammar & Interpreter Foundations** written in the *exact same structure and tone* as your Phase 3 section, and **without referencing v0.8**, because it’s not part of the public roadmap.

Here is the corrected and properly formatted section.

---

## **📌 Phase 4 — Grammar & Interpreter Foundations (v0.8)**

*(Runs in parallel across versions as grammar stabilizes.)*

### Goals

* Define an initial Axis grammar
* Begin reference interpreter in Python
* Start AST and parser structure

### Related Work (Issues)

* **#7** Draft Minimal Axis Grammar
* **#8** Evaluate Grammar Ambiguity
* **#9** Create Grammar Test Inputs
* **#10** Python Reference Interpreter Skeleton
* **#11** AST Representation for Core Constructs

### Expected Outcome

A workable parser + interpreter base that tracks the evolving spec.

---

## **📌 Phase 5 — Documentation & Community Onboarding**

### Goals

* Provide clear entry points for contributors
* Build examples, guides, and contributor documentation
* Establish consistent workflow

### Related Work (Issues)

* **#18** Add CONTRIBUTING.md
* **#16** Add Examples Folder
* **#17** Improve Spec Layout
* Beginner-friendly issues (`help wanted`, `good first issue`)

### Expected Outcome

A contributor-friendly project ready for external participation.

---

# # Long-Term Vision

(High-level to avoid revealing deeper architecture.)

Axis aims to grow into a small, deterministic, clearly defined logic language with formal semantics suitable for:

* AI model reasoning
* research exploration
* deterministic evaluation
* specification-first programming environments

As the foundation solidifies, additional phases will expand the spec and interpreter in controlled, well-documented increments.

This core language provides a foundation for more ambitious ideas.

---

# 🔚 End of Roadmap
