![Frames](./docs/assets/header-long.png)

<h6 align="center">
    <a href="./LICENSE.md">License</a>
  · <a href="https://docs.rs/frames">Docs</a>
</h6>

<p align="center">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/frames?color=C9CBFF&logoColor=D9E0EE&style=for-the-badge">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/frames?color=F2CDCD&logoColor=D9E0EE&style=for-the-badge">
</p>

<hr/>

You want to move your robot from point A to point B. That's all you ever want to do, really. The question is, how do you model the multiple frames that exist in your environment?

Say you have the following:

![Example figure](./docs/assets/fig-a.png)

In short — `frames` solves for `(x, y, θ)` like so:

```rust
use frames::prelude::*;
use nalgebra::{Isometry2, Vector2};
use std::f32::consts::PI;

fn main() -> Result<(), FrameError> {
    let mut frames = Frames::new();

    let field = Frame::new("field");
    let robot = Frame::new("robot");

    frames.add_frame(field, Isometry2::new(Vector2::new(0., 0.), 0.))?;
    frames.add_frame(robot, Isometry2::new(Vector2::new(1., 1.), PI))?;

    let x = Point::new("x");
    frames.add_point_in_context(
        x,
        Isometry2::new(Vector2::new(7., 5.), PI),
        field,
    )?;

    assert_eq!(
        frames.get_point_in_context(x, robot)?,
        Isometry2::new(Vector2::new(6., 4.), 0.)
    );

    Ok(())
}
```

## Features

 * **Efficient** — optimized, no-nonsense calculations.
 * **Scalable** — calculate in any dimensions.
 * **Type-safe** — errors can be seen at compile-time.
