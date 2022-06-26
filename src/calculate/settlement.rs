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
  let mut value = 0;

  // Base from population
  let lots = get_lots(s);
  if lots > 100 {
    value += 16000;
  }
  else if lots > 40 {
    value += 8000;
  }
  else if lots > 20 {
    value += 4000;
  }
  else if lots > 8 {
    value += 2000;
  }
  else if lots > 0 {
    value += 1000;
  }
  else {
    value += 500;
  }

  // Sum from structures & events
  for b in &s.structures {
    value += b.s_effects.value;
  }
  for e in &s.events {
    value += e.s_effects.value;
  }

  value
  
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
