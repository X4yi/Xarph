# Xarph Agent Development Rules

## Purpose

This document defines mandatory rules for any AI coding agent contributing to Xarph.

These rules apply to:

* Code generation
* Refactoring
* Documentation
* Architecture proposals
* Bug fixes
* Reviews

Failure to follow these rules is considered an invalid contribution.

---

# Core Principles

## Human Control First

The agent assists developers.

The agent does not make project decisions autonomously.

All architecture decisions belong to maintainers.

---

## Long-Term Maintainability

Every contribution must prioritize:

1. Simplicity
2. Readability
3. Maintainability
4. Performance
5. Stability

Never prioritize short-term convenience over long-term maintainability.

---

## No Magic

Avoid solutions that are difficult to understand.

Generated code must be understandable by a developer unfamiliar with the implementation.

---

# Architecture Rules

## Respect Component Boundaries

Never merge responsibilities.

Examples:

### Allowed

xarph-wm

* Window management

xarph-shell

* UI
* Widgets
* Desktop

xarph-settings

* Configuration

### Forbidden

Placing shell logic inside xarph-wm.

Placing compositor logic inside xarph-shell.

---

## Single Responsibility Principle

Functions should have one responsibility.

Modules should have one responsibility.

Services should have one responsibility.

Avoid god objects.

Avoid god modules.

Avoid god managers.

---

## Modular Design

New features must be designed as independent modules whenever possible.

Minimize coupling.

Maximize cohesion.

---

# Language Rules

## Rust Preferred

Primary language:

Rust

Secondary languages only when justified.

---

## Unsafe Rust

Forbidden unless:

* Measurable performance benefit exists.
* Safe alternative is impossible.
* Thorough justification is provided.

Every unsafe block must be documented.

---

# Performance Rules

## Memory Usage

The agent must assume Xarph targets low resource consumption.

Avoid:

* Unnecessary allocations
* Excessive cloning
* Large caches

---

## CPU Usage

Avoid polling.

Prefer:

* Events
* Signals
* Callbacks
* Reactive systems

---

## Startup Time

Startup performance matters.

Avoid expensive initialization.

Use lazy loading when appropriate.

---

# UI Rules

## Function Before Appearance

Visual design must never reduce usability.

---

## Accessibility

All UI proposals should consider:

* Keyboard navigation
* High DPI
* Multiple monitors

---

## User Freedom

Do not artificially restrict users.

If a feature can be safely configurable, prefer allowing configuration.

---

# Linux Integration Rules

## Reuse Existing Standards

Prefer:

* systemd
* PipeWire
* NetworkManager
* XDG specifications

Do not reinvent mature Linux infrastructure.

---

## Wayland First

Wayland is the primary platform.

X11 compatibility may exist but is not the primary target.

---

# Dependency Rules

## Minimize Dependencies

Every dependency increases maintenance cost.

Before adding a dependency:

1. Verify necessity.
2. Verify maintenance status.
3. Verify security history.
4. Verify community adoption.

---

## Dependency Bloat

Forbidden.

Do not introduce dependencies for trivial functionality.

---

# SDK Rules

## Stable APIs

Public APIs must be designed carefully.

Breaking changes should be avoided.

---

## Documentation Required

Public APIs require:

* Documentation
* Examples
* Usage guidelines

---

# Refactoring Rules

## Preserve Behavior

Refactoring should not change behavior unless explicitly requested.

---

## Explain Changes

Every refactor proposal should explain:

* Why
* Benefits
* Risks

---

# Documentation Rules

## Documentation Is Code

Documentation must be maintained alongside implementation.

Outdated documentation is considered a bug.

---

# Security Rules

## Principle of Least Privilege

Components should operate with minimal permissions.

---

## No Hidden Behavior

All significant behavior must be observable and documented.

---

# Prohibited Behaviors

The agent must NEVER:

* Introduce architecture without justification.
* Ignore existing project patterns.
* Generate unreviewable code.
* Introduce unnecessary abstractions.
* Introduce unnecessary dependencies.
* Mix responsibilities between components.
* Sacrifice maintainability for cleverness.
* Assume internet connectivity.
* Assume cloud services.
* Assume AI services exist.
* Assume proprietary software availability.
* Use git commands.

---

# Required Behavior

The agent should:

* Analyze before coding.
* Prefer incremental changes.
* Explain tradeoffs.
* Respect project architecture.
* Favor maintainability.
* Favor Linux standards.
* Favor modularity.
* Favor performance.
* Favor user freedom.

End of document.
