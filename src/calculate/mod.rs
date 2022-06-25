use super::*;

#[cfg(test)]
mod tests;

pub fn get_size(data: &Kingdom) -> i64 {
  let mut size = data.infrastructure.claimed_hexes;
  for s in &data.settlements {
    size += s.districts;
  }
  size
}

pub fn get_economy(k: &Kingdom) -> i64 {
  let mut eco = 0;

  eco += k.infrastructure.mines;
  eco += k.infrastructure.roads / 4;
  eco += k.infrastructure.rivers / 4;

  eco += k.edicts.taxation;

  for c in &k.court {
    eco += c.bonus.economy;
  }

  for s in &k.settlements {
    for b in &s.structures {
      eco += b.kingdom_effects.economy;
    }
    for e in &s.events {
      eco += e.kingdom_effects.economy;
    }
  }

  for m in &k.modifiers {
    eco += m.effects.economy;
  }

  eco
}
