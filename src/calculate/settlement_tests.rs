use super::*;

#[test]
fn corruption() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.corruption = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.corruption = 1;
  s.events.push(e);

  assert_eq!(
    get_corruption(&s),
    2,
    "Corruption is based on Structures and Events"
  );
}

#[test]
fn crime() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.crime = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.crime = 1;
  s.events.push(e);

  assert_eq!(
    get_crime(&s),
    2,
    "Crime is based on Structures and Events"
  );
}

#[test]
fn law() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.law = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.law = 1;
  s.events.push(e);

  assert_eq!(
    get_law(&s),
    2,
    "Law is based on Structures and Events"
  );
}

#[test]
fn lore() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.lore = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.lore = 1;
  s.events.push(e);

  assert_eq!(
    get_lore(&s),
    2,
    "Lore is based on Structures and Events"
  );
}

#[test]
fn productivity() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.productivity = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.productivity = 1;
  s.events.push(e);

  assert_eq!(
    get_productivity(&s),
    2,
    "Productivity is based on Structures and Events"
  );
}

#[test]
fn society() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.s_effects.society = 1;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.society = 1;
  s.events.push(e);

  assert_eq!(
    get_society(&s),
    2,
    "Society is based on Structures and Events"
  );
}

#[test]
fn lots() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.lots = 2;
  s.structures.push(b);
  let mut b = Structure::default();
  b.lots = 2;
  s.structures.push(b);

  assert_eq!(
    get_lots(&s),
    4,
    "Lots are the sum of lots for all structures."
  );
}

#[test]
fn defence() {
  let mut s = Settlement::default();
  s.walls = 3;

  let mut b = Structure::default();
  b.s_effects.defense = 2;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.defense = 1;
  s.events.push(e);

  assert_eq!(
    get_defense(&s),
    6,
    "Defense is based on walls, Structures and Events"
  );
}

#[test]
fn purchase_limit() {
  let mut s = Settlement::default();
  s.size = SettlementSize::SmallTown;

  let mut e = Event::default();
  e.s_effects.purchase_limit_multiplier = 1.1;
  s.events.push(e);
  let e = Event::default();
  s.events.push(e);

  let mut b = Structure::default();
  b.s_effects.purchase_limit_multiplier = 1.1;
  s.structures.push(b);
  let b = Structure::default();
  s.structures.push(b);

  assert_eq!(
    get_purchase_limit(&s),
    6000,
    "Purchase limit is based on settlement's Size with multiplier bonus"
  );
}

#[test]
fn base_value() {
  let mut s = Settlement::default();
  s.size = SettlementSize::SmallTown;

  let mut b = Structure::default();
  b.s_effects.value = 500;
  b.s_effects.value_multiplier = 1.1;
  s.structures.push(b);
  let b = Structure::default();
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.value = 500;
  e.s_effects.value_multiplier = 1.1;
  s.events.push(e);
  let e = Event::default();
  s.events.push(e);

  assert_eq!(
    get_value(&s),
    2200,
    "Base Value is based on settlement's Size, Structures and Events"
  );
}

#[test]
fn base_limit() {
  let mut s = Settlement::default();
  s.size = SettlementSize::Hamlet;

  assert_eq!(
    get_limit(&s),
    200,
    "Base Limit is based on settlement's Size"
  );
}

#[test]
fn size_estimate() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.lots = 5;
  s.structures.push(b);

  assert_eq!(
    get_size_estimate(&s),
    SettlementSize::SmallTown,
    "Size estimate is based on number of lots (population = around 250 * lots)"
  );
}
