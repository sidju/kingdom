use super::*;

#[cfg(test)]
mod kingdom_tests;

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
      eco += b.k_effects.economy;
    }
    for e in &s.events {
      eco += e.k_effects.economy;
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
      loyalty += b.k_effects.loyalty;
    }
    for e in &s.events {
      loyalty += e.k_effects.loyalty;
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
      stab += b.k_effects.stability;
    }
    for e in &s.events {
      stab += e.k_effects.stability;
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
      inc += b.k_effects.income;
    }
    for e in &s.events {
      inc += e.k_effects.income;
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
      con += b.k_effects.consumption;
    }
    for e in &s.events {
      con += e.k_effects.consumption;
    }
  }

  for m in &k.modifiers {
    con += m.effects.consumption;
  }

  con
}

pub fn get_fame(k: &Kingdom) -> i64 {
  let mut fame = 0;
  // Lore and society need to be summed, as they contribute at factor 1/10
  let mut ls = 0;

  for s in &k.settlements {
    for b in &s.structures {
      fame += b.k_effects.fame;
      ls += b.s_effects.lore + b.s_effects.society;
    }
    for e in &s.events {
      fame += e.k_effects.fame;
      ls += e.s_effects.lore + e.s_effects.society;
    }
  }
  fame += ls / 10;

  for m in &k.modifiers {
    fame += m.effects.fame;
  }

  fame
}

pub fn get_infamy(k: &Kingdom) -> i64 {
  let mut infamy = 0;
  // Crime and Corruption need to be summed, as they contribute at factor 1/10
  let mut cc = 0;

  for s in &k.settlements {
    for b in &s.structures {
      infamy += b.k_effects.infamy;
      cc += b.s_effects.crime + b.s_effects.corruption;
    }
    for e in &s.events {
      infamy += e.k_effects.infamy;
      cc += e.s_effects.crime + e.s_effects.corruption;
    }
  }
  infamy += cc / 10;

  for m in &k.modifiers {
    infamy += m.effects.infamy;
  }

  infamy
}
