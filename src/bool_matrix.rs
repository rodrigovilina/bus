#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoolMatrix {
  pub width: usize,
  pub height: usize,
  data: Vec<u8>,
}

impl BoolMatrix {
  pub fn new(width: usize, height: usize) -> Self {
    let size: usize = (width * height + 7) / 8;

    Self {
      width,
      height,
      data: vec![0; size],
    }
  }

  pub const fn data(&self) -> &Vec<u8> {
    &self.data
  }

  const fn calculate_index(&self, x: usize, y: usize) -> (usize, u8) {
    let index = y * self.width + x;
    (index / 8, 1 << (index % 8))
  }

  pub fn set(&mut self, x: usize, y: usize, value: bool) {
    let (byte_index, bit_mask) = self.calculate_index(x, y);
    if value {
      self.data[byte_index] |= bit_mask; // Set bit to 1.
    } else {
      self.data[byte_index] &= !bit_mask; // Set bit to 0.
    }
  }

  pub fn get(&self, x: usize, y: usize) -> bool {
    let (byte_index, bit_mask) = self.calculate_index(x, y);
    (self.data[byte_index] & bit_mask) != 0
  }

  pub fn column(&self, col_index: usize) -> Self {
    let mut column = Self::new(1, self.height);
    for y in 0..self.height {
      let value = self.get(col_index, y);
      column.set(0, y, value);
    }
    column
  }

  pub fn row(&self, row_index: usize) -> Self {
    let mut row = Self::new(self.width, 1);
    for x in 0..self.width {
      let value = self.get(x, row_index);
      row.set(x, 0, value);
    }
    row
  }

  pub fn set_row_range(&mut self, row: usize, from: usize, to: usize, value: bool) {
    assert!(
      from <= to,
      "The 'from' index must be less than or equal to the 'to' index."
    );
    assert!(
      to < self.width,
      "The 'to' index must be within the row width."
    );

    for x in from..=to {
      self.set(x, row, value);
    }
  }

  pub fn try_add(one: &BoolMatrix, two: &BoolMatrix) -> Option<Self> {
    if one.width != two.width || one.height != two.height {
      return None;
    }

    let mut result = Self::new(one.width, two.height);

    for (i, (&byte_self, &byte_rhs)) in one.data.iter().zip(two.data.iter()).enumerate() {
      if byte_self & byte_rhs == 0 {
        result.data[i] = byte_self | byte_rhs;
      } else {
        // return Err("Overlap detected in BoolMatrix::try_add");
        return None;
      }
    }

    Some(result)
  }

  pub fn has_colliding_bits(&self, other: &Self) -> bool {
    assert!(!(self.width != other.width || self.height != other.height));
    self
      .data
      .iter()
      .zip(other.data.iter())
      .any(|(&byte_self, &byte_other)| byte_self & byte_other != 0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let mut bm: BoolMatrix = BoolMatrix::new(8, 8);
    assert_eq!(bm.data().len(), 8);
    assert!(!bm.get(0, 0));
    bm.set(0, 0, true);
    assert!(bm.get(0, 0));
  }
}
