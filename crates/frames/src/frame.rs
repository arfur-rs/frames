use nalgebra::Isometry2;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Frames<'a> {
    frames: HashMap<Frame<'a>, Isometry2<f32>>,
    points: HashMap<Point<'a>, Isometry2<f32>>,
}

impl<'a> Frames<'a> {
    pub fn new() -> Self {
        Self {
            frames: HashMap::new(),
            points: HashMap::new(),
        }
    }

    pub fn add_frame(&mut self, f: Frame<'a>, c: Isometry2<f32>) -> Result<(), FrameError> {
        self.frames
            .insert(f, c)
            .map_or_else(|| Ok(()), |_| Err(FrameError::AlreadyExists))
    }

    /// Adds a point in the master context.
    pub fn add_point(&mut self, p: Point<'a>, c: Isometry2<f32>) -> Result<(), FrameError> {
        self.points
            .insert(p, c)
            .map_or_else(|| Ok(()), |_| Err(FrameError::AlreadyExists))
    }

    /// Adds a point in a specific context.
    pub fn add_point_in_context(
        &mut self,
        p: Point<'a>,
        c: Isometry2<f32>,
        f: Frame<'a>,
    ) -> Result<(), FrameError> {
        self.points
            .insert(p, c * self.get_frame(f)?.to_owned())
            .map_or_else(|| Ok(()), |_| Err(FrameError::AlreadyExists))
    }

    pub fn get_frame(&self, f: Frame<'a>) -> Result<Isometry2<f32>, FrameError> {
        self.frames
            .get(&f)
            .ok_or(FrameError::CouldNotFindFrame)
            .map(ToOwned::to_owned)
    }

    pub fn get_point(&self, p: Point<'a>) -> Result<Isometry2<f32>, FrameError> {
        self.points
            .get(&p)
            .ok_or(FrameError::CouldNotFindPoint)
            .map(ToOwned::to_owned)
    }

    pub fn get_point_in_context(
        &mut self,
        p: Point<'a>,
        f: Frame<'a>,
    ) -> Result<Isometry2<f32>, FrameError> {
        let offset = self.get_frame(f)?;
        self.get_point(p).map(|x| x / offset)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Frame<'a> {
    name: &'a str,
}

impl<'a> Frame<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, Default)]
pub struct Point<'a> {
    name: &'a str,
}

impl<'a> Point<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("frame already exists")]
    AlreadyExists,
    #[error("could not find frame")]
    CouldNotFindFrame,
    #[error("could not find point")]
    CouldNotFindPoint,
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    use super::*;

    #[test]
    fn point_from_frame_to_frame() -> Result<(), FrameError> {
        let some_rotation = std::f32::consts::FRAC_PI_2;

        let field = Frame::new("field");
        let robot = Frame::new("robot");
        let mut frames = Frames::new();

        frames.add_frame(field, Isometry2::new(Vector2::new(0., 0.), 0.))?;
        frames.add_frame(robot, Isometry2::new(Vector2::new(1., 1.), some_rotation))?;
        dbg!(&frames);

        let x = Point::new("x");
        frames.add_point_in_context(
            x,
            Isometry2::new(Vector2::new(5., 5.), some_rotation),
            field,
        )?;
        dbg!(&frames);

        assert_eq!(
            frames.get_point_in_context(x, robot)?,
            Isometry2::new(Vector2::new(4., 4.), 0.)
        );

        Ok(())
    }
}
