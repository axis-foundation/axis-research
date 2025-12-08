# CONTRIBUTING.md

**Axis Project — Contribution Guidelines (v0.1)**

Axis is an early-stage research project exploring new approaches to AI-assisted programming.
Contributions are welcome, and this guide explains how to participate effectively.

---

## 1. Branch Naming Convention

Use the format:

```
<type>/<scope>--<short-description>
```

**Types:**

* `feat`
* `fix`
* `spec`
* `docs`
* `refactor`
* `ops`

**Examples:**

```
spec/core--update-grammar
docs/readme--clarify-goals
feat/compiler--initial-parser
```

---

## 2. Specification Change Workflow

For changes to specification documents, please open an Issue first so the change can be discussed before a PR is submitted.

---

## 3. Pull Requests

PRs should:

* Reference an Issue
* Follow the branch naming convention
* Contain only related changes
* Pass relevant checks once CI is established

Major design discussions should start in Issues or Discussions before opening PRs.

---

## 4. Coding Guidelines (Early Stage)

As the project grows, more formal style guides will emerge.
For now:

### General

* Prefer deterministic and explicit behavior
* Avoid unnecessary abstractions
* Write code that is easy to reason about for both humans and tooling

### Python

* Prioritize clarity
* Document assumptions

### Rust

* Use idiomatic error handling
* Avoid `unsafe` unless required and well-documented

---

## 5. Project Philosophy (High-Level Summary)

Axis explores:

* AI-readable logical structures
* Deterministic transformation pipelines
* Minimal surface area for reasoning

These principles help keep the system approachable and consistent.

---

## 6. Code of Conduct

Axis follows the standard GitHub Contributor Covenant.

---

## 7. Future Updates

As the project evolves, this guide will expand with:

* More detailed style rules
* Tooling workflows
* Additional contribution areas

The goal is to maintain clarity while enabling collaboration.

---