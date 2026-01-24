<!--
Sync Impact Report
==================
Version Change: 0.1.0 → 0.2.0
Modified Principles: None
Added Sections:
  - Development Workflow > Branch Strategy (base branch, feature branches, lifecycle, merge strategy, integration rules)
Removed Sections: None
Templates Status:
  ✅ .specify/templates/plan-template.md - aligned, no changes needed
  ✅ .specify/templates/spec-template.md - aligned, no changes needed
  ✅ .specify/templates/tasks-template.md - aligned, no changes needed
  ✅ .cursor/commands/*.md - reviewed, speckit.specify already enforces branch naming
  ✅ README.md - no updates needed
Follow-up TODOs: None
Note: Version 0.2.0 - Added branch strategy guidance for trunk-based development
-->

# lazybook Constitution

## Core Principles

### I. Specification-First Development

All features MUST begin with a complete specification document that defines user scenarios, requirements, and success criteria before any technical planning or implementation begins. Specifications MUST be technology-agnostic and focus on WHAT users need and WHY, never HOW to implement. Every specification MUST pass quality validation through the checklist system before proceeding to planning phase.

**Rationale**: Clear requirements prevent scope creep, reduce rework, and ensure all stakeholders share a common understanding of feature goals before investing in design and code.

### II. Modular Planning

Implementation plans MUST separate concerns into distinct phases: research (Phase 0), design artifacts including data models and API contracts (Phase 1), and task breakdown (Phase 2). Each phase MUST produce documented outputs that inform subsequent phases. Plans MUST explicitly identify and resolve all technical uncertainties through research before design begins.

**Rationale**: Phased planning reduces complexity, enables early validation of assumptions, and produces artifacts that serve as living documentation throughout the feature lifecycle.

### III. Independent User Stories

Features MUST be decomposed into independently testable user stories, each prioritized by business value. Every user story MUST be implementable, testable, and deployable without dependencies on other stories of the same feature. Tasks MUST be organized by user story to enable incremental delivery of value.

**Rationale**: Independent stories enable parallel development, early feedback loops, and the ability to ship minimum viable increments without waiting for complete feature sets.

### IV. Test-Driven Development (NON-NEGOTIABLE)

When tests are included in a feature specification, they MUST be written before implementation, MUST fail initially, and MUST pass after implementation completes. The Red-Green-Refactor cycle is strictly enforced. Contract tests MUST verify library interfaces, integration tests MUST cover cross-component interactions, and unit tests MUST validate individual components.

**Rationale**: TDD ensures requirements are testable, reduces defects, documents expected behavior, and enables confident refactoring.

### V. Documentation as Code

All project artifacts (specifications, plans, contracts, data models, quickstart guides) MUST be versioned alongside code in markdown format. Documentation MUST be generated through command workflows, never manually maintained in isolation. Agent context files MUST be automatically updated when new technologies are introduced.

**Rationale**: Version-controlled documentation stays synchronized with code, enables audit trails, and facilitates knowledge transfer across team members and AI agents.

### VI. Domain-Driven Design Architecture

All feature implementations MUST follow Domain-Driven Design principles. Domain logic MUST be isolated in a pure domain layer free of infrastructure concerns. Bounded contexts MUST be explicitly identified and their boundaries enforced through module structure. Value objects MUST be preferred over primitive types for domain concepts. Aggregates MUST protect invariants and define transaction boundaries.

**Rationale**: DDD ensures business logic remains maintainable and testable by separating domain concerns from technical implementation details. Clear bounded contexts prevent coupling and enable independent evolution of subsystems.

### VII. TUI-First Interface Design

The primary user interface MUST be a Text-based User Interface (TUI). All interactive features MUST be accessible through keyboard navigation. UI state management MUST be separated from domain logic. TUI components MUST be composable and reusable. The interface MUST provide clear visual feedback for all user actions and system states.

**Rationale**: TUI interfaces provide efficient keyboard-driven workflows for power users, work consistently across terminal environments, and minimize system resource usage while maintaining rich interactivity.

## Technical Constraints

### Technology Stack

- **Language**: Rust (stable toolchain)
- **Interface**: Text-based User Interface (TUI)
- **Architecture**: Domain-Driven Design
- **Project-Specific Tooling**: Managed exclusively through mise

### Tool Management

All project-specific development tools MUST be declared in `.mise.toml` or `.tool-versions` configuration. Team members MUST use mise to install and manage tool versions. Global system tools MUST NOT be assumed; all required tools MUST be explicitly versioned in mise configuration. CI/CD pipelines MUST use mise to ensure consistent tool versions across environments.

**Rationale**: Centralized tool version management through mise eliminates "works on my machine" problems, ensures reproducible builds, and simplifies onboarding.

### Rust Development Standards

Code MUST pass `cargo clippy` with zero warnings. Code MUST be formatted with `rustfmt` using project configuration. All public APIs MUST have documentation comments. Unsafe code MUST be explicitly justified with safety invariants documented. Error types MUST implement `std::error::Error` and provide context through error chains.

**Rationale**: Consistent code quality standards reduce maintenance burden, improve code review efficiency, and catch common bugs early.

### Privacy and Security

This project manages private personal information. All data MUST be encrypted at rest. Sensitive data MUST NOT be logged. Authentication credentials MUST be stored securely using platform keyring integration where available. All data access MUST be auditable. Security-sensitive code MUST undergo explicit review.

**Rationale**: Personal information management requires stringent security controls to protect user privacy and prevent data breaches.

## Development Workflow

### Command-Driven Process

All feature work MUST flow through the SpecKit command chain: `/speckit.constitution` → `/speckit.specify` → `/speckit.plan` → `/speckit.tasks` → `/speckit.implement`. Each command MUST validate its prerequisites before execution and MUST produce outputs that serve as inputs to subsequent commands. Branches MUST follow the naming convention `###-feature-name` where ### is an incrementing number.

### Quality Gates

Each phase MUST pass validation before proceeding:

- **Specification phase**: Quality checklist MUST be complete with all items passing or explicitly justified
- **Planning phase**: Constitution check MUST pass or complexity violations MUST be documented with rationale
- **Implementation phase**: All tests MUST be written and failing before code is written

### Artifact Organization

Feature artifacts MUST be organized in `specs/###-feature-name/` directories containing: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, `contracts/`, and `tasks.md`. Checklists MUST be stored in `specs/###-feature-name/checklists/`. Source code structure MUST follow the conventions defined in the implementation plan.

### Branch Strategy

**Base Branch**: `main` is the single long-lived branch representing the stable, releasable state of the project.

**Feature Branches**: All feature work MUST be done in dedicated feature branches following the naming convention `###-feature-name` where `###` is an incrementing sequential number and `feature-name` is a short, kebab-case description (2-4 words). Feature branches MUST be created from and merged back into `main`.

**Branch Lifecycle**:
- Feature branches are created automatically by `/speckit.specify` command
- Feature branches MUST be deleted after successful merge to `main`
- Feature branches SHOULD be short-lived (complete and merge within reasonable timeframe)
- Stale feature branches (inactive >30 days) SHOULD be reviewed for closure or rebase

**Merge Strategy**:
- Feature branches MUST be merged into `main` via pull request (when applicable) or direct merge (for solo development)
- Commits SHOULD be squashed if they represent incremental work-in-progress rather than logical units
- Merge commits MAY be used to preserve feature branch history when beneficial
- The commit history on `main` MUST remain clean and meaningful

**Integration Rules**:
- `main` MUST always be in a working state (all tests pass, code compiles)
- Breaking changes during `0.x.y` versions MAY be merged to `main` but MUST be documented
- After `1.0.0` release, breaking changes MUST be carefully managed and communicated

**Rationale**: A simple trunk-based approach with feature branches minimizes merge conflicts, accelerates integration, and aligns with the solo/small team nature of this project while maintaining code quality through the SpecKit workflow.

## Governance

### Amendment Procedure

Constitution amendments MUST be versioned using semantic versioning (MAJOR.MINOR.PATCH). Changes MUST be documented in a Sync Impact Report listing modified principles, added sections, removed sections, and affected templates. All dependent templates and command files MUST be reviewed and updated to maintain consistency with amended principles.

### Versioning Policy

- **MAJOR version**: Backward incompatible changes to governance or principle removal/redefinition
- **MINOR version**: New principles added or existing principles materially expanded
- **PATCH version**: Clarifications, wording improvements, or typo corrections without semantic change

**Version 0.x.y (Initial Development Phase)**: While the constitution is in version 0.x.y, the project is in initial development. During this phase, breaking changes MAY occur in MINOR versions (0.x.0). Once the constitution and project reach stability, version 1.0.0 will be released, after which the standard semantic versioning rules apply strictly.

### Compliance Review

All feature work MUST verify compliance with this constitution at planning phase through the Constitution Check section. Violations MUST be explicitly justified in the Complexity Tracking table with rationale explaining why simpler alternatives were rejected. Unjustified violations MUST result in feature rejection until compliance is achieved.

### Living Document

This constitution supersedes all other development practices and conventions. When conflicts arise between this constitution and other guidance, this constitution takes precedence. The constitution MUST be reviewed and amended as project needs evolve, with all changes propagated to dependent artifacts.

**Version**: 0.2.0 | **Ratified**: 2026-01-24 | **Last Amended**: 2026-01-24
