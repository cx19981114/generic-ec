## v0.2.4
* Add `generic_ec::multiscalar` which helps optimizing multiscalar multiplication [#29]

[#29]: https://github.com/dfns/generic-ec/pull/29

## v0.2.3
* Add `generic_ec::serde::PreferCompact` that serializes points/scalars in compact form,
  but deserialization recognizes both compact and non-compact formats [#28]

[#28]: https://github.com/dfns/generic-ec/pull/28

## v0.2.2
* Implement `serde_with::SerializeAs<&T>` for `generic_ec::serde::Compact` when `T` is
  serializable via `Compact` [#27]

[#27]: https://github.com/dfns/generic-ec/pull/27

## v0.2.1
* Make `generic_ec::serde` module always available even when `serde` feature is disabled [#25]

[#25]: https://github.com/dfns/generic-ec/pull/25

## v0.2.0

All changes prior to this version weren't documented
