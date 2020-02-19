// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::Rem;
use std::ops::Sub;


/// Round `value` to the next multiple of `round_to`, unless it already
/// is a multiple.
pub fn round_up<T>(
  value: T,
  round_to: T,
) -> <T as Add<<<T as Sub<<T as Rem>::Output>>::Output as Rem<T>>::Output>>::Output
where
  T: Copy,
  T: Rem<T>,
  T: Sub<<T as Rem>::Output>,
  T: Add<<<T as Sub<<T as Rem>::Output>>::Output as Rem<T>>::Output>,
  <T as Sub<<T as Rem>::Output>>::Output: Rem<T>,
{
  value + ((round_to - (value % round_to)) % round_to)
}


#[cfg(test)]
pub mod tests {
  use super::*;


  #[test]
  fn rounding() {
    assert_eq!(round_up(1, 10), 10);
    assert_eq!(round_up(10, 100), 100);
    assert_eq!(round_up(99, 100), 100);
    assert_eq!(round_up(100, 100), 100);
    assert_eq!(round_up(5, 1000), 1000);
    assert_eq!(round_up(1, 1000), 1000);
    assert_eq!(round_up(0, 1000), 0);
  }
}
