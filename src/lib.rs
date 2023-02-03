//! # plotters-arrows
//!
//! Simple arrow shapes for [plotters](::plotters) crate.

use plotters::element::{Drawable, PointCollection};
use plotters::prelude::*;
use plotters::style::SizeDesc;
use plotters_backend::{BackendCoord, DrawingErrorKind};

#[derive(Clone)]
pub struct ThinArrow<Coord, Size: SizeDesc> {
    points: [Coord; 2],
    head: Size,
    width: Size,
    style: ShapeStyle,
}

impl<Coord> ThinArrow<Coord, i32> {
    pub fn new(nock: Coord, tip: Coord, style: impl Into<ShapeStyle>) -> Self {
        Self {
            points: [nock, tip],
            head: 5,
            width: 5,
            style: style.into(),
        }
    }
}

impl<Coord, Size: SizeDesc> ThinArrow<Coord, Size> {
    pub fn new_detail(
        nock: Coord,
        tip: Coord,
        head: Size,
        width: Size,
        style: impl Into<ShapeStyle>,
    ) -> Self {
        Self {
            points: [nock, tip],
            head,
            width,
            style: style.into(),
        }
    }

    pub fn head(self, head: Size) -> Self {
        Self { head, ..self }
    }

    pub fn width(self, width: Size) -> Self {
        Self { width, ..self }
    }
}

impl<'a, Coord: 'a, Size: SizeDesc> PointCollection<'a, Coord> for &'a ThinArrow<Coord, Size> {
    type Point = &'a Coord;

    type IntoIter = &'a [Coord];

    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}

impl<DB: DrawingBackend, Coord, Size: SizeDesc> Drawable<DB> for ThinArrow<Coord, Size> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let nock = pos.next();
        let tip = pos.next();
        debug_assert!(nock.is_some());
        debug_assert!(tip.is_some());
        if let (Some(nock), Some(tip)) = (nock, tip) {
            let head = self.head.in_pixels(&parent_dim) as f64;
            let width = self.width.in_pixels(&parent_dim) as f64;
            if nock != tip {
                let dx = (tip.0 - nock.0) as f64;
                let dy = (tip.1 - nock.1) as f64;
                let d = f64::sqrt(dx * dx + dy * dy);
                let head = head.min(d);
                let width = width.min(d);
                let half_width = width * 0.5;
                let left = (
                    tip.0 + ((-head * dx + half_width * dy) / d) as i32,
                    tip.1 + ((-head * dy - half_width * dx) / d) as i32,
                );
                let right = (
                    tip.0 + ((-head * dx - half_width * dy) / d) as i32,
                    tip.1 + ((-head * dy + half_width * dx) / d) as i32,
                );
                backend.draw_line(nock, tip, &self.style)?;
                backend.draw_line(tip, left, &self.style)?;
                backend.draw_line(tip, right, &self.style)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct TriangleArrow<Coord, Size: SizeDesc> {
    points: [Coord; 2],
    head: Size,
    width: Size,
    style: ShapeStyle,
}

impl<Coord> TriangleArrow<Coord, i32> {
    pub fn new(nock: Coord, tip: Coord, style: impl Into<ShapeStyle>) -> Self {
        Self {
            points: [nock, tip],
            head: 5,
            width: 5,
            style: style.into(),
        }
    }
}

impl<Coord, Size: SizeDesc> TriangleArrow<Coord, Size> {
    pub fn new_detail(
        nock: Coord,
        tip: Coord,
        head: Size,
        width: Size,
        style: impl Into<ShapeStyle>,
    ) -> Self {
        Self {
            points: [nock, tip],
            head,
            width,
            style: style.into(),
        }
    }

    pub fn head(self, head: Size) -> Self {
        Self { head, ..self }
    }

    pub fn width(self, width: Size) -> Self {
        Self { width, ..self }
    }
}

impl<'a, Coord: 'a, Size: SizeDesc> PointCollection<'a, Coord> for &'a TriangleArrow<Coord, Size> {
    type Point = &'a Coord;

    type IntoIter = &'a [Coord];

    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}

impl<DB: DrawingBackend, Coord, Size: SizeDesc> Drawable<DB> for TriangleArrow<Coord, Size> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        let nock = pos.next();
        let tip = pos.next();
        debug_assert!(nock.is_some());
        debug_assert!(tip.is_some());
        if let (Some(nock), Some(tip)) = (nock, tip) {
            let head = self.head.in_pixels(&parent_dim) as f64;
            let width = self.width.in_pixels(&parent_dim) as f64;
            if nock != tip {
                let dx = (tip.0 - nock.0) as f64;
                let dy = (tip.1 - nock.1) as f64;
                let d = f64::sqrt(dx * dx + dy * dy);
                let head = head.min(d);
                let width = width.min(d);
                let half_width = width * 0.5;
                let left = (
                    tip.0 + ((-head * dx + half_width * dy) / d) as i32,
                    tip.1 + ((-head * dy - half_width * dx) / d) as i32,
                );
                let right = (
                    tip.0 + ((-head * dx - half_width * dy) / d) as i32,
                    tip.1 + ((-head * dy + half_width * dx) / d) as i32,
                );
                backend.draw_line(nock, tip, &self.style)?;
                backend.fill_polygon(vec![tip, left, right, tip], &self.style)?;
            }
        }
        Ok(())
    }
}
