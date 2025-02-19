# Changelog

## Unreleased

### Added

- Added an option to eagerly evaluate entity attributes and re-use across calls to `is_authorized`
- Adds APIs to `Entities` to make it easy to add a collection of entities to an existing `Entities` structure
- Export the `cedar_policy_core::evaluator::{EvaluationError, EvaluationErrorKind}` and
  `cedar_policy_core::authorizer::AuthorizationError` error types.
- Added an API to `ParseError` to quickly get the primary source span
- Added an API, `unknown_entities`, to `PolicySet` to collect unknown entity UIDs from `PartialResponse`.
- Added APIs `remove`, `remove_template` and `unlink` to remove policies from the `PolicySet`
- Added API `get_linked_policies` to get the policies linked to a `Template`

### Changed

- Renamed `cedar_policy_core::est::EstToAstError` to `cedar_policy_core::est::FromJsonError`
- Renamed `cedar_policy_core::entities::JsonDeserializationError::ExtensionsError` to `cedar_policy_core::entities::JsonDeserializationError::FailedExtensionsFunctionLookup`.
- Renamed variants in `cedar_policy::SchemaError`
- The `Diagnostics::errors()` function now returns an iterator over `AuthorizationError`s.
- The `Response::new()` constructor now expects a `Vec<AuthorizationError>` as its third argument.
- Implements RFC #19, making validation slightly more strict, but more explainable.
- Improved formatting for error messages.
- Changed the semantics of equality for IP ranges. For example,
  `ip("192.168.0.1/24") == ip("192.168.0.3/24")` was previously `true` and is now
  `false`. The behavior of equality on single IP addresses is unchanged, and so is
  the behavior of `.isInRange()`.
- Standardized on duplicates being errors instead of last-write-wins in the following APIs:
	+ Policy set JSONs
	+ Template set JSONs
	+ Template instantiation records
	+ Entity slice JSONs
	+ Context JSONs
- `<EntityId as FromStr>::Error` is now `Infallible` instead of `ParseErrors`
- Fixed bug (#370) related to how the validator handles template-linked policies 


## 2.4.1

### Added
- New experimental API to construct queries with `Unknown` fields for partial evaluation.

### Changed
- Improved validation error messages for access to undeclared attributes and
  unsafe access to optional attributes to report the target of the access (fix #175).
- `EntityUid`'s impl of `FromStr` is no longer marked as deprecated.
- Fixed #299, condition of `if` not being partial evaluated.
- Update the behavior of `Request::principal()`, `Request::action()`, and
  `Request::resource()` to return `None` if the entities are unspecified (i.e.,
  constructed by passing `None` to `Request::new()`).

## 2.4.0

### Added
- New methods exported for `EntityTypeName`.
  - `basename` to get the basename (without namespaces).
  - `namespace_components` to get the namespace as an iterator over its components.
  - `namespace` to get the namespace as a single string.

### Changed
- Some error types now carry more information about the error, with error
messages updated appropriately. For instance, added list of attributes that _do_
exist to the `RecordAttrDoesNotExist` error message.
- Improved error messages for some schema type parsing errors.
  - When an entity type shape or action context is declared with type other than
  `Record`, the error message will indicated the affected entity type or action.
- Improved a variety of other error messages
- Increased precision for validating records.  Previously,
`permit(principal, action, resource) when {{"foo": 5} has bar};` would validate.
Now it will not, since we know `{"foo": 5} has bar` is `False`, and the
validator will return an error for a policy that can never fire.
- Removed deprecated `__expr` escapes from integration tests.

## 2.3.3

### Added
- Re-export `cedar_policy_core::entities::EntitiesError`.
- Fixed bug (#150) around implicit namespaces for actions in `memberOf` lists in
  schemas. An action without an explicit namespace in a `memberOf` now
  correctly uses the default namespace.

### Changed
- Improved error messages and documentation for some errors raised during
  policy parsing, validation, and evaluation.
- More precise "expected tokens" lists in some parse errors.

## 2.3.2

### Removed
- Move public API for partial evaluation behind experimental feature flag. To
  continue using this feature you must enable the `partial-eval` feature flag.

### Changed

- Improved error detection in schema based parsing (fix issues #73, #74).
  - Detect entities with parents of an incorrect entity type.
  - Detect entities with an undeclared entity type.
- Slightly improved error text on some validation type errors
- Improved error messages for some schema type parsing errors
  - Parsing a schema type without the `"type"` field will generate an error
    stating that `"type"` is a required field instead of an inscrutable error
    complaining about the untagged enum `SchemaType`.
  - Parsing a schema type with a `"type"` field corresponding to one of the
    builtin types but missing a required field for that type will generate an
    error stating that a required field is missing instead of claiming that it
    could not find "common types" definition for that builtin type.

## 2.3.1

### Fixed

- Fix a panic in `PolicySet::link()` that could occur when the function was called
  with a policy id corresponding to a static policy.

## 2.3.0

### Changed

- Implementation of
[RFC 9](https://github.com/cedar-policy/rfcs/blob/main/text/0009-disallow-whitespace-in-entityuid.md)
which disallows embedded whitespace, comments, and control characters in the
inputs to several Rust API functions including `EntityTypeName::from_str()` and
`EntityNamespace::from_str()`, as well as in some fields of the Cedar JSON
schema format (e.g., namespace declarations, entity type names), Cedar JSON
entities format (e.g., entity type names, extension function names) and the
Cedar JSON policy format used by `Policy::from_json()` (e.g., entity type names,
extension function names). The risk that this may be a breaking change for some
Cedar users was accepted due to the potential security ramifications; see
discussion in the RFC.

## 2.2.0

### Added

- `Entities::write_to_json` function to api.rs

## 2.1.0

### Added

- `Schema::action_entities` to provide access to action entities defined in a schema.

### Changed

- Update `cedar-policy-core` dependency.

### Fixed

- Resolve warning in `Cargo.toml` due to having both `license` and `license-file` metadata entries.

## 2.0.3

### Fixed

- Update `Cargo.toml` metadata to correctly represent this crate as Apache-2.0 licensed.

## 2.0.2

## 2.0.1

## 2.0.0

Initial release of `cedar-policy`.
