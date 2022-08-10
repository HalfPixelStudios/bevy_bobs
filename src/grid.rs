//! Grid related utilities

use anyhow::{anyhow, Result};
use bevy::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridError {
    #[error("tried to access position outside of grid {0}")]
    OutOfBounds(IVec2),
}

/// Collection of grid positions that can be queried and manipulated
///
/// The grid is a read-only structure. It is not a source of truth. GridPositions are the actual
/// source of truth. The grid is just a visual representation of where all the GridPosition objects
/// are relative to each other.
pub struct Grid<T: PartialEq> {
    width: i32,
    height: i32,
    grid: Vec<Vec<T>>,
}

impl<T: PartialEq> Grid<T> {
    /// Construct a new grid
    pub fn new(width: i32, height: i32) -> Self {
        // TODO this is sorta stupid lol (maybe just force T to derive Clone)
        let grid_vec = (0..width * height).into_iter().map(|_| vec![]).collect();

        Grid {
            width,
            height,
            grid: grid_vec,
        }
    }

    /// Check if position is within the grid
    pub fn bounds_check(&self, pos: &IVec2) -> bool {
        0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height
    }

    pub fn pos_to_index(&self, pos: &IVec2) -> Result<usize> {
        if self.bounds_check(pos) {
            Ok((pos.y * self.width + pos.x) as usize)
        } else {
            Err(anyhow!(GridError::OutOfBounds(pos.to_owned())))
        }
    }

    /// Get reference to cell at position
    pub fn get_cell(&self, pos: &IVec2) -> Result<&Vec<T>> {
        let ind = self.pos_to_index(pos)?;
        // shouldn't panic (since already bounds checked)?
        Ok(self.grid.get(ind).unwrap())
    }

    /// Get mutable reference to cell at position
    pub fn get_cell_mut(&mut self, pos: &IVec2) -> Result<&mut Vec<T>> {
        let ind = self.pos_to_index(pos)?;
        Ok(self.grid.get_mut(ind).unwrap())
    }

    /// Insert a cell entity at position
    pub fn insert_at(&mut self, pos: &IVec2, val: T) -> Result<()> {
        self.get_cell_mut(pos)?.push(val);
        Ok(())
    }

    /// Query if cell contains a given entity
    pub fn contains_at(&self, pos: &IVec2, val: T) -> Result<bool> {
        Ok(self.get_cell(pos)?.contains(&val))
    }

    /// Query if a cell at position is empty
    pub fn empty_at(&self, pos: &IVec2) -> Result<bool> {
        Ok(self.get_cell(pos)?.is_empty())
    }

    /// Wipe the entire map
    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.clear();
        }
    }

    /// Get grid width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get grid height
    pub fn height(&self) -> i32 {
        self.height
    }
}
