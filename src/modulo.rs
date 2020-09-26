// Copyright (C) 2020 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::Rem;


/// Calculate `x` modulo `y`.
pub fn modulo<T>(x: T, y: T) -> <<<T as Rem>::Output as Add<T>>::Output as Rem<T>>::Output
where
  T: Copy + Rem<T>,
  <T as Rem>::Output: Add<T>,
  <<T as Rem>::Output as Add<T>>::Output: Rem<T>,
{
  ((x % y) + y) % y
}


#[cfg(test)]
pub mod tests {
  use super::*;


  #[test]
  fn modulo_results() {
    assert_eq!(modulo(-4, 3), 2);
    assert_eq!(modulo(-3, 3), 0);
    assert_eq!(modulo(-2, 3), 1);
    assert_eq!(modulo(-1, 3), 2);
    assert_eq!(modulo(0, 3), 0);
    assert_eq!(modulo(1, 3), 1);
    assert_eq!(modulo(2, 3), 2);
    assert_eq!(modulo(3, 3), 0);
    assert_eq!(modulo(4, 3), 1);
    assert_eq!(modulo(5, 3), 2);
  }
}
