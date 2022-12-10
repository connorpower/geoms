# `::geoms`

[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[Documentation (`main` branch)][docs-main-url]

## Overview

Geometry for Microsoft platforms - a set of geometry primitives with memory
layouts optimized for native APIs (Win32, Direct2D, and Direct3D).

The goal of this crate is to provide an idiomatic but zero-cost interface to
common geometric types used in Microsoft graphics APIs such as Direct2D,
Direct3D and native Win32.

## Current Status

Only a small number of primitives have been implemented as required for personal
projects. The API is unstable and expected to change.

## Features

If _feature_ `"d2d"` is enabled, then some primitives can be directly
converted into a Direct2D structures.

If _feature_ `"win32"` is enabled, then some primitives can be directly
converted into a Win32 structures.

## Usage

To use `geoms`, add the following to your `Cargo.toml`:

```toml
[dependencies]
geoms = "0.0.1"
```

To enable optional conversions to native Microsoft types, activate the
appropriate features. E.g. for Direct2D support:

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
```


[docs-main-url]: http://connorpower.com/geoms/
[docs-url]: https://docs.rs/geoms
[docs-badge]: https://docs.rs/geoms/badge.svg
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: http://connorpower.com/geoms/main/LICENSE
[actions-url]: https://github.com/connorpower/geoms/actions?query=workflow%3ACI
[actions-badge]: https://github.com/connorpower/geoms/workflows/CI/badge.svg
