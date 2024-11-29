## Deprecated

This crate is deprecated. The functionality has been integrated into Bevy.
Use [`Gizmos::cross`](https://docs.rs/bevy/latest/bevy/prelude/struct.Gizmos.html#method.cross) for 3D
and [`Gizmos::cross_2d`](https://docs.rs/bevy/latest/bevy/prelude/struct.Gizmos.html#method.cross_2d) for 2D.

_____

# bevy-cross-gizmo

Adds `cross` and `cross_2d` gizmos to Bevy.

![cross](cross.png)
![cross_2d](cross_2d.png)

## Installation

```toml
[dependencies]
bevy-cross-gizmo = "0.14.0"
```

## Usage

```rust
use bevy::prelude::*;
use bevy_cross_gizmo::BevyCrossGizmo;

fn update(mut gizmos: Gizmos) {
    gizmos.cross(Vec3::ZERO, Quat::IDENTITY, 0.3, Color::WHITE);
    gizmos.cross_2d(Vec3::ZERO, 0.0, 0.3, Color::WHITE);
}
```

## Compatibility

| `bevy` | `bevy-cross-gizmo`  |
|--------|---------------------|
| 0.15.0 | part of Bevy Gizmos |
| 0.14.0 | 0.14.0              |
| 0.13.2 | 0.13.2              |

## License

Dual-licensed under either:

* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
