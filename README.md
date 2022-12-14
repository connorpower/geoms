# geoms

[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

Geometry for Microsoft platforms - a set of geometry primitives with memory
layouts optimized for native APIs ([Win32][], [Direct2D][], and [Direct3D][]).

The goal of this crate is to provide an idiomatic but zero-cost interface to
common geometric types used in Microsoft graphics APIs. Integration with the
excellent [`::num_traits`][] crate allows for geometric types to be represented
by
 arbitrary numeric types, and allows conversion between different numeric
representations of entire higher-level types.

[→ Documentation][docs-main-url]

## Optional Features

- If _feature_ `"d2d"` is enabled, then some primitives can be directly
converted into [Direct2D][] structures.
- If _feature_ `"win32"` is enabled, then some primitives can be directly
converted into [Win32][] structures.

## Usage

To use `geoms`, add the following to your `Cargo.toml`:

```toml
[dependencies]
geoms = "0.0.1"
```

To enable optional conversions to native Microsoft types, activate the
appropriate features. E.g. for [Direct2D][] support:

```toml
[dependencies]
geoms = { version = "0.0.1", features = ["d2d"] }
```

### Example

```rust
use ::geoms::d2::{Rect2D, Point2D, Size2D};
use ::windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;

// Construct our Rust rectangle, 100x20 pixels at point 10,10
let rect = Rect2D::with_size_and_origin(
    Size2D { width: 100.0, height: 20.0 },
    Point2D { x: 10.0, y: 10.0 },
);

// Convert our Rust rectangle into a Direct2D rectangle. This merely
// transmutes under the hood as the memory layouts are the same.
let d2d_rect: D2D_RECT_F = rect.into();

// Confirm our Direct2D rectangle has the expected properties.
assert_eq!(rect.left, 10.0);
assert_eq!(rect.right, 110.0);

// Cast our entire rect to a u32 representation of the same primitive:
let u_rect = rect.cast::<u32>();
assert_eq!(u_rect.left, 10);
assert_eq!(u_rect.right, 110);

```

## Current Status

Only a small number of primitives have been implemented as required for personal
projects. The API is unstable and expected to change.

## License

This project is licensed under the [MIT license][mit-url]

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository by you, shall be licensed as MIT, without any
additional terms or conditions.


[docs-main-url]: http://connorpower.com/geoms/
[docs-url]: https://docs.rs/geoms
[docs-badge]: https://docs.rs/geoms/badge.svg
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/connorpower/geoms/blob/main/LICENSE
[actions-url]: https://github.com/connorpower/geoms/actions?query=workflow%3ACI
[actions-badge]: https://github.com/connorpower/geoms/workflows/CI/badge.svg
[Direct2D]: https://learn.microsoft.com/en-us/windows/win32/direct2d/direct2d-overview
[Direct3D]: https://learn.microsoft.com/en-us/windows/win32/getting-started-with-direct3d
[Win32]: https://learn.microsoft.com/en-us/windows/win32/
[`::num_traits`]: https://crates.io/crates/num-traits
