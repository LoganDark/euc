pub mod triangles;

pub use self::triangles::Triangles;

use crate::Pipeline;

/// A trait that represents types that turn vertex streams into fragment coordinates.
///
/// Rasterizers take an iterator of vertices and emit fragment positions. They do not, by themselves, perform shader
/// execution, depth testing, etc.
pub trait Rasterizer {
    /// Rasterize the given vertices into fragments.
    ///
    /// - `target_size`: The size of the render target(s) in pixels
    /// - `principal_x`: Whether the rasterizer should prefer the x axis as the principal iteration access (see
    ///   [`Texture::principle_axes`])
    /// - `emit_fragment`: The function that should be called with the target coordinate (in pixels), weights for each
    ///   vertex as a contribution to the final interpolated vertex output, the vertex outputs, and the depth of each
    ///   rasterized fragment.
    ///
    /// # Safety
    ///
    /// `emit_fragment` must only be called with fragment positions that are valid for the `target_size` parameter
    /// provided. Undefined behaviour can be assumed to occur if this is not upheld.
    unsafe fn rasterize<P, I, F>(
        &self,
        pipeline: &P,
        vertices: I,
        target_size: [usize; 2],
        principal_x: bool,
        emit_fragment: F,
    )
    where
        P: Pipeline,
        I: Iterator<Item = ([f32; 4], P::VsOut)>,
        F: FnMut([usize; 2], &[f32], &[P::VsOut], f32);
}
