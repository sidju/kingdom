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
fn value() {
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.lots = 2;
  b.s_effects.value = 500;
  s.structures.push(b);

  let mut e = Event::default();
  e.s_effects.value = 500;
  s.events.push(e);

  assert_eq!(
    get_value(&s),
    2000,
    "Value is based on Structures and Events"
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
