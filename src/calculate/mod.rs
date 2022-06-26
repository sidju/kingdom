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

pub fn get_loyalty(k: &Kingdom) -> i64 {
  let mut loyalty = 0;

  for c in & k.court {
    loyalty += c.bonus.loyalty;
  }

  for s in &k.settlements {
    for b in &s.structures {
      loyalty += b.kingdom_effects.loyalty;
    }
    for e in &s.events {
      loyalty += e.kingdom_effects.loyalty;
    }
  }

  for m in &k.modifiers {
    loyalty += m.effects.loyalty;
  }

  loyalty
}

pub fn get_stability(k: &Kingdom) -> i64 {
  let mut stab = 0;

  stab += k.infrastructure.quarries;
  stab += k.infrastructure.sawmills;
  stab += k.infrastructure.roads / 8;
  stab += k.infrastructure.rivers / 8;

  for c in &k.court {
    stab += c.bonus.stability;
  }

  for s in &k.settlements {
    for b in &s.structures {
      stab += b.kingdom_effects.stability;
    }
    for e in &s.events {
      stab += e.kingdom_effects.stability;
    }
  }

  for m in &k.modifiers {
    stab += m.effects.stability;
  }

  stab
}

pub fn get_income(k: &Kingdom) -> i64 {
  let mut inc = 0;

  for s in &k.settlements {
    for b in &s.structures {
      inc += b.kingdom_effects.income;
    }
    for e in &s.events {
      inc += e.kingdom_effects.income;
    }
  }

  for m in &k.modifiers {
    inc += m.effects.income;
  }

  inc
}
