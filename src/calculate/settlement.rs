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

pub fn get_size_estimate(s: &Settlement) -> SettlementSize {
  // Estimate through population based on lots
  // Since events should affect population and not all lots are likely to house
  // the same density of population the simple estimate of 250 people per lot is
  // likely to be overruled by GM as often as not.
  let estimated_pop = get_lots(s) * 250;
  if estimated_pop > 25_000 {
    SettlementSize::Metropolis
  }
  else if estimated_pop > 10_000 {
    SettlementSize::LargeCity
  }
  else if estimated_pop > 5_000 {
    SettlementSize::SmallCity
  }
  else if estimated_pop > 2_000 {
    SettlementSize::LargeTown
  }
  else if estimated_pop > 200 {
    SettlementSize::SmallTown
  }
  // The estimate is too imprecise to guess below village
  else {
    SettlementSize::Village
  }
}

pub fn get_value(s: &Settlement) -> i64 {
  // Base from settlement size
  let mut value = match s.size {
    SettlementSize::Thorpe => 50,
    SettlementSize::Hamlet => 200,
    SettlementSize::Village => 500,
    SettlementSize::SmallTown => 1_000,
    SettlementSize::LargeTown => 2_000,
    SettlementSize::SmallCity => 4_000,
    SettlementSize::LargeCity => 8_000,
    SettlementSize::Metropolis => 16_000,
  };

  // Sum from structures & events
  for b in &s.structures {
    value += b.s_effects.value;
  }
  for e in &s.events {
    value += e.s_effects.value;
  }

  value
}

pub fn get_limit(s: &Settlement) -> i64 {
  // Mapped from settlement size
  match s.size {
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
  match s.size {
    SettlementSize::Thorpe => 500,
    SettlementSize::Hamlet => 1_000,
    SettlementSize::Village => 2_500,
    SettlementSize::SmallTown => 5_000,
    SettlementSize::LargeTown => 10_000,
    SettlementSize::SmallCity => 25_000,
    SettlementSize::LargeCity => 50_000,
    SettlementSize::Metropolis => 100_000,
  }
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
