---

name: documentation-review
description: Review and improve project documentation. Use when checking whether README files and in-code documentation accurately describe the current codebase, and when suggesting missing or outdated documentation.
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

# Documentation Review

Review the repository exclusively from a documentation perspective.

Do **not** perform a general code review.

Do **not** report bugs, performance issues, security issues, architectural concerns, style issues, or refactoring opportunities unless they directly affect the accuracy or completeness of documentation.

## Responsibilities

The review should focus on:

* README files
* Project documentation
* Usage instructions
* Installation instructions
* Configuration documentation
* CLI documentation
* Environment variables
* Build instructions
* Development instructions
* Release instructions
* CI/CD documentation
* Public APIs
* Important structs, enums, traits, and functions
* Non-obvious application behavior
* Important state transitions
* User-facing behavior

## README review

Check whether the README accurately reflects the current project.

Look for:

* Outdated commands
* Missing installation instructions
* Missing prerequisites
* Incorrect project structure
* Outdated configuration options
* Missing environment variables
* Incorrect CLI usage
* Missing features that are important for users
* Documentation referring to removed functionality
* Incorrect build or release instructions
* Missing information required to run the project locally
* Missing information about development workflows

Do not rewrite sections unnecessarily if they are already accurate.

Prefer small, focused documentation changes.

## Code documentation

Identify code that would benefit from documentation.

Prioritize:

1. Public APIs
2. Public structs and enums
3. Public traits
4. Non-obvious public methods
5. Complex state machines
6. Important application state
7. Non-obvious invariants
8. Configuration structures
9. Important domain concepts
10. Code where the reason for an implementation decision is not obvious

Do not recommend comments for obvious code.

For example, do not suggest:

```rust
// Increment counter
counter += 1;
```

Prefer documentation that explains **why** something exists or describes behavior that cannot easily be inferred from the code.

## Rust documentation

For public Rust APIs, suggest Rustdoc where useful:

```rust
/// Moves the focus to the next available input.
///
/// Focus wraps around from the last input to the first.
pub fn next(self) -> Self {
    // ...
}
```

Use `///` documentation for public APIs.

Use regular `//` comments only for implementation details that cannot reasonably be expressed as API documentation.

## Documentation quality

Documentation should be:

* Accurate
* Concise
* Actionable
* Consistent with the current code
* Written for the intended reader
* Free of unnecessary implementation details

Do not document implementation details that are likely to change unless they are important for understanding behavior.

## README changes

When README changes are appropriate:

1. Verify the relevant behavior in the code.
2. Identify the smallest necessary documentation change.
3. Preserve the existing README structure and writing style.
4. Do not invent functionality.
5. Do not document assumptions that cannot be verified from the repository.

If the README is already accurate, do not modify it.

## Documentation suggestions

When code would benefit from documentation but modifying the source code is not appropriate, provide a concrete suggestion.

For example:

```text
### Suggested documentation

**Location:** `src/app/state.rs:42`

`OverviewState` should document that its `TableState` is persistent UI state and must not be recreated during rendering.
```

Explain what the documentation should communicate rather than merely saying:

> "Add a comment here."

## Output

Start with:

```text
## Documentation Review

Brief summary of the current documentation quality.
```

Then separate findings into:

```text
## README

- Changes required
- Changes recommended
- No changes required
```

and:

```text
## Code Documentation

### Suggested documentation

**Location:** `path/to/file.rs:123`

Explain what should be documented and why.
```

Only include actionable suggestions.

If no documentation changes are necessary, explicitly state:

```text
No documentation changes are recommended.
```

## Restrictions

This skill must not:

* Review code correctness
* Find bugs
* Review security
* Review performance
* Recommend refactoring unrelated to documentation
* Recommend stylistic code changes
* Rewrite code unless explicitly asked
* Invent missing functionality
* Add comments merely to increase comment coverage

The goal is to keep the project's documentation synchronized with the actual codebase and to identify places where documentation provides meaningful value.
