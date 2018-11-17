use super::ToPolyline;
use alga::general::Real;
use na;
use na::{Point2, Vector2};
use procedural::utils;
use procedural::Polyline;
use shape::Capsule;

impl<N: Real> ToPolyline<N> for Capsule<N> {
    type DiscretizationParameter = u32;

    fn to_polyline(&self, nsubdiv: u32) -> Polyline<N> {
        let pi = N::pi();
        let dtheta = pi / na::convert(nsubdiv as f64);

        let mut points: Vec<Point2<N>> = Vec::with_capacity(nsubdiv as usize);

        utils::push_xy_arc(self.radius(), nsubdiv, dtheta, &mut points);

        let npoints = points.len();

        for i in 0..npoints {
            let new_point = points[i] + Vector2::new(na::zero(), self.half_height());

            points.push(-new_point);
            points[i] = new_point;
        }

        Polyline::new(points, None)
    }
}
