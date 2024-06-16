# bevy-cross-gizmo

Adds `cross` and `cross_2d` gizmos to Bevy.

![cross](cross.png)
![cross_2d](cross_2d.png)

## Installation

```toml
[dependencies]
bevy-cross-gizmo = "0.13.2"
```

## Usage

```rust
use bevy::prelude::*;
use bevy_cross_gizmo::BevyCrossGizmo;

fn update(mut gizmos: Gizmos) {
    gizmos.cross(Vec3::ZERO, 0.3, Color::WHITE);
    gizmos.cross_2d(Vec3::ZERO, 0.3, Color::WHITE);
}
```

## Compatibility

| `bevy` | `bevy-cross-gizmo` |
|--------|--------------------|
| 0.13.2 | 0.13.2             |
