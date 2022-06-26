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

  inc += k.infrastructure.quarries;
  inc += k.infrastructure.sawmills;
  inc += k.infrastructure.mines;

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

pub fn get_consumption(k: &Kingdom) -> i64 {
  let mut con = 0;

  con += k.infrastructure.claimed_hexes;
  con -= k.infrastructure.farms * 2;
  con -= k.infrastructure.fisheries;

  for s in &k.settlements {
    con += s.districts;
    for b in &s.structures {
      con += b.kingdom_effects.consumption;
    }
    for e in &s.events {
      con += e.kingdom_effects.consumption;
    }
  }

  for m in &k.modifiers {
    con += m.effects.consumption;
  }

  con
}
