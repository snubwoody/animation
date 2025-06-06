use winit::dpi::PhysicalSize;

/// Represents the `width` and `height` of any structure
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Size<N> {
    pub width: N,
    pub height: N,
}

impl<N> Size<N>
where
    N: Copy,
{
    pub fn new(width: N, height: N) -> Self {
        Self { width, height }
    }

    pub fn unit(value: N) -> Self {
        Self::new(value, value)
    }
}

impl<N> From<PhysicalSize<N>> for Size<N> {
    fn from(value: PhysicalSize<N>) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
