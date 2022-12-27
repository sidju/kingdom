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

pub fn get_crime(s: &Settlement) -> i64 {
  let mut crime = 0;

  for b in &s.structures {
    crime += b.s_effects.crime;
  }
  for e in &s.events {
    crime += e.s_effects.crime;
  }

  crime
}

pub fn get_law(s: &Settlement) -> i64 {
  let mut law = 0;

  for b in &s.structures {
    law += b.s_effects.law;
  }
  for e in &s.events {
    law += e.s_effects.law;
  }

  law
}

pub fn get_lore(s: &Settlement) -> i64 {
  let mut lore = 0;

  for b in &s.structures {
    lore += b.s_effects.lore;
  }
  for e in &s.events {
    lore += e.s_effects.lore;
  }

  lore
}

pub fn get_productivity(s: &Settlement) -> i64 {
  let mut prod = 0;

  for b in &s.structures {
    prod += b.s_effects.productivity;
  }
  for e in &s.events {
    prod += e.s_effects.productivity;
  }

  prod
}

pub fn get_society(s: &Settlement) -> i64 {
  let mut society = 0;

  for b in &s.structures {
    society += b.s_effects.society;
  }
  for e in &s.events {
    society += e.s_effects.society;
  }

  society
}

pub fn get_lots(s: &Settlement) -> i64 {
  let mut lots = 0;

  for b in &s.structures {
    lots += b.lots;
  }

  lots
}

pub fn get_value(s: &Settlement) -> i64 {
  // Base from settlement size
  let base_value = match get_settlement_size(&s) {
    SettlementSize::Thorpe => 50,
    SettlementSize::Hamlet => 200,
    SettlementSize::Village => 500,
    SettlementSize::SmallTown => 1_000,
    SettlementSize::LargeTown => 2_000,
    SettlementSize::SmallCity => 4_000,
    SettlementSize::LargeCity => 8_000,
    SettlementSize::Metropolis => 16_000,
  };

  let mut value = 0;
  // Sum from structures & events
  for b in &s.structures {
    value += b.s_effects.value;
    value += ((b.s_effects.value_multiplier - 1.0) * base_value as f64) as i64;
  }
  for e in &s.events {
    value += e.s_effects.value;
    value += ((e.s_effects.value_multiplier - 1.0) * base_value as f64) as i64;
  }

  base_value + value
}

pub fn get_limit(s: &Settlement) -> i64 {
  // Mapped from settlement size
  match get_settlement_size(&s) {
    SettlementSize::Thorpe => 50,
    SettlementSize::Hamlet => 200,
    SettlementSize::Village => 500,
    SettlementSize::SmallTown => 1_000,
    SettlementSize::LargeTown => 2_000,
    SettlementSize::SmallCity => 4_000,
    SettlementSize::LargeCity => 8_000,
    SettlementSize::Metropolis => 16_000,
  }
}

pub fn get_purchase_limit(s: &Settlement) -> i64 {
  // Mapped from settlement size
  let base_limit = match get_settlement_size(&s) {
    SettlementSize::Thorpe => 500,
    SettlementSize::Hamlet => 1_000,
    SettlementSize::Village => 2_500,
    SettlementSize::SmallTown => 5_000,
    SettlementSize::LargeTown => 10_000,
    SettlementSize::SmallCity => 25_000,
    SettlementSize::LargeCity => 50_000,
    SettlementSize::Metropolis => 100_000,
  };
  let mut limit = 0;
  for b in &s.structures {
    limit += ((b.s_effects.purchase_limit_multiplier - 1.0) * base_limit as f64) as i64;
  }
  for e in &s.events {
    limit += ((e.s_effects.purchase_limit_multiplier - 1.0) * base_limit as f64) as i64;
  }

  base_limit + limit
}

pub fn get_defense(s: &Settlement) -> i64 {
  let mut defense = 0;

  defense += s.walls;
  for b in &s.structures {
    defense += b.s_effects.defense;
  }
  for e in &s.events {
    defense += e.s_effects.defense;
  }

  defense
}

pub fn get_settlement_size(s: &Settlement) -> SettlementSize {
  if s.population > 25_000 {
    SettlementSize::Metropolis
  }
  else if s.population > 10_000 {
    SettlementSize::LargeCity
  }
  else if s.population > 5_000 {
    SettlementSize::SmallCity
  }
  else if s.population > 2_000 {
    SettlementSize::LargeTown
  }
  else if s.population > 200 {
    SettlementSize::SmallTown
  }
  else if s.population > 60 {
    SettlementSize::Village
  }
  else if s.population > 20 {
    SettlementSize::Hamlet
  }
  else {
    SettlementSize::Thorpe
  }
}
