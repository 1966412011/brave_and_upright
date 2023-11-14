use ilattice::glam::UVec3;
use ilattice::extent::Extent;
use ndshape::{ConstShape3u32, Shape};

pub const CHUNK_LENGTH: u32 = 32;
pub const CHUNK_LENGTH_U: usize = 32;
pub const CHUNK_SHAPE: ConstShape3u32<CHUNK_LENGTH, CHUNK_LENGTH, CHUNK_LENGTH> =
    ConstShape3u32::<CHUNK_LENGTH, CHUNK_LENGTH, CHUNK_LENGTH>;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq,Default)]
pub struct Voxel(pub u8);

pub struct Chunk {
    pub data: Box<[Voxel]>,
}

impl Chunk {
    pub fn new(initial_val: Voxel) -> Self {
        Self {
            data: vec![initial_val; CHUNK_LENGTH_U * CHUNK_LENGTH_U * CHUNK_LENGTH_U]
                .into_boxed_slice(),
        }
    }

    #[inline]
    pub fn new_empty() -> Self {
        Self {
            data: vec![Default::default(); CHUNK_LENGTH_U * CHUNK_LENGTH_U * CHUNK_LENGTH_U]
                .into_boxed_slice(),
        }
    }

    #[inline]
    pub fn voxel_at(&self, pos: UVec3) -> Voxel {
        self.data[CHUNK_SHAPE.linearize(pos.to_array()) as usize]
    }

    #[inline]
    pub fn voxel_at_mut(&mut self, pos: UVec3) -> &mut Voxel {
        &mut self.data[CHUNK_SHAPE.linearize(pos.to_array()) as usize]
    }

    #[inline]
    pub fn fill_extent(&mut self, extent: Extent<UVec3>, val: Voxel) {
        ndcopy::fill3(
            extent.shape.to_array(),
            val,
            &mut self.data,
            &CHUNK_SHAPE,
            extent.minimum.to_array(),
        );
    }
}
