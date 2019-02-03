use crate::bounding_volume::aabb_utils;
use crate::bounding_volume::{HasBoundingVolume, AABB};
use crate::math::Isometry;
use na::Real;
use crate::shape::ConvexHull;

impl<N: Real> HasBoundingVolume<N, AABB<N>> for ConvexHull<N> {
    #[inline]
    fn bounding_volume(&self, m: &Isometry<N>) -> AABB<N> {
        aabb_utils::point_cloud_aabb(m, self.points())
    }
}
