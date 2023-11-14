use super::chunk::{Chunk, Voxel, CHUNK_LENGTH, CHUNK_SHAPE};
use bevy::{math::IVec3, prelude::Resource};
use ilattice::{morton::Morton3i32, vector::Map as VecMap};
use ndshape::Shape;
use std::collections::BTreeMap;

#[derive(Resource)]
pub struct ChunkMap {
    chunks: BTreeMap<Morton3i32, Chunk>,
    shape_mask: IVec3,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self {
            chunks: BTreeMap::new(),
            shape_mask: !(IVec3::from(CHUNK_SHAPE.as_array().map(|x| x as i32)) - IVec3::ONE),
        }
    }

    #[inline]
    pub fn voxel_at(&self, pos: IVec3) -> Option<Voxel> {
        let chunk_minimum = pos & self.shape_mask;
        let local_minimum = ilattice::glam::IVec3::from(pos.to_array())
            .map(|x| x.rem_euclid(CHUNK_LENGTH as i32))
            .as_uvec3();

        self.buffer_at(chunk_minimum)
            .map(|buffer| buffer.voxel_at(local_minimum))
    }

    #[inline]
    pub fn voxel_at_mut(&mut self, pos: IVec3) -> Option<&mut Voxel> {
        let chunk_minimum = pos & self.shape_mask;
        let local_minimum = ilattice::glam::IVec3::from(pos.to_array())
            .map(|x| x.rem_euclid(CHUNK_LENGTH as i32))
            .as_uvec3();

        self.buffer_at_mut(chunk_minimum)
            .map(|buffer| buffer.voxel_at_mut(local_minimum))
    }

    #[inline]
    pub fn buffer_at(&self, minimum: IVec3) -> Option<&Chunk> {
        let minimum = ilattice::glam::IVec3::from(minimum.to_array());
        self.chunks.get(&minimum.into())
    }

    #[inline]
    pub fn buffer_at_mut(&mut self, minimum: IVec3) -> Option<&mut Chunk> {
        let minimum = ilattice::glam::IVec3::from(minimum.to_array());
        self.chunks.get_mut(&minimum.into())
    }

    #[inline]
    pub fn exists(&self, minimum: IVec3) -> bool {
        let minimum = ilattice::glam::IVec3::from(minimum.to_array());
        self.chunks.contains_key(&minimum.into())
    }

    pub fn insert(&mut self, minimum: IVec3, buffer: Chunk) {
        let minimum = ilattice::glam::IVec3::from(minimum.to_array());

        self.chunks.insert(minimum.into(), buffer);
    }

    pub fn insert_empty(&mut self, minimum: IVec3) {
        let minimum = ilattice::glam::IVec3::from(minimum.to_array());
        self.chunks.insert(
            minimum.into(),
            Chunk::new_empty(),
        );
    }

    pub fn remove(&mut self, pos: IVec3) -> Option<Chunk> {
        let pos = ilattice::glam::IVec3::from(pos.to_array());
        self.chunks.remove(&pos.into())
    }

    #[inline]
    pub const fn shape_mask(&self) -> IVec3 {
        self.shape_mask
    }
}
