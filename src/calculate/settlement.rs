use super::*;

pub fn get_corruption(s: &Settlement) -> i64 {
  let mut corr = 0;

  for b in &s.structures {
    corr += b.s_effects.corruption;
  }
  for e in &s.events {
    corr += e.s_effects.corruption;
  }

  corr
}
