use super::*;

#[test]
fn size() {
  let mut k = Kingdom::default();
  k.infrastructure.claimed_hexes = 4;
  let mut s = Settlement::default();
  s.districts = 1;
  k.settlements.push(s);
  let mut s = Settlement::default();
  s.districts = 2;
  k.settlements.push(s);
  assert_eq!(
    get_size(&k),
    7,
    "The size of a kingdom is claimed hexes + total nr of settlement districts."
  );
}

#[test]
fn economy() {
  let mut k = Kingdom::default();
  k.infrastructure.mines = 1;
  k.infrastructure.roads = 8;
  k.infrastructure.rivers = 16;

  let mut c = Courtier::default();
  c.bonus.economy = 4;
  k.court.push(c);

  // Start building settlement
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.kingdom_effects.economy = 1;
  s.structures.push(b);

  let mut e = SettlementEvent::default();
  e.kingdom_effects.economy = 1;
  s.events.push(e);

  // Done building settlement, add to kingdom
  k.settlements.push(s);

  let mut m = KingdomModifier::default();
  m.effects.economy = 1;
  k.modifiers.push(m);

  assert_eq!(
    get_economy(&k),
    14,
    "Economy is based on Court, Structures, Events and KingdomModifiers"
  );
}

#[test]
fn loyalty() {
  let mut k = Kingdom::default();

  let mut c = Courtier::default();
  c.bonus.loyalty = 4;
  k.court.push(c);

  // Start building settlement
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.kingdom_effects.loyalty = 1;
  s.structures.push(b);

  let mut e = SettlementEvent::default();
  e.kingdom_effects.loyalty = 1;
  s.events.push(e);

  // Done building settlement, add to kingdom
  k.settlements.push(s);

  let mut m = KingdomModifier::default();
  m.effects.loyalty = 1;
  k.modifiers.push(m);

  assert_eq!(
    get_loyalty(&k),
    7,
    "Loyalty is based on Court, Structures, Events and KingdomModifiers"
  );
}

#[test]
fn stability() {
  let mut k = Kingdom::default();
  k.infrastructure.sawmills = 1;
  k.infrastructure.quarries = 1;
  k.infrastructure.roads = 8;
  k.infrastructure.rivers = 16;

  let mut c = Courtier::default();
  c.bonus.stability = 4;
  k.court.push(c);

  // Start building settlement
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.kingdom_effects.stability = 1;
  s.structures.push(b);

  let mut e = SettlementEvent::default();
  e.kingdom_effects.stability = 1;
  s.events.push(e);

  // Done building settlement, add to kingdom
  k.settlements.push(s);

  let mut m = KingdomModifier::default();
  m.effects.stability = 1;
  k.modifiers.push(m);

  assert_eq!(
    get_stability(&k),
    12,
    "Stability is based on Court, Structures, Events and KingdomModifiers"
  );
}

#[test]
fn income() {
  let mut k = Kingdom::default();
  k.infrastructure.quarries = 1;
  k.infrastructure.sawmills = 1;
  k.infrastructure.mines = 1;

  // Start building settlement
  let mut s = Settlement::default();

  let mut b = Structure::default();
  b.kingdom_effects.income = 1;
  s.structures.push(b);

  let mut e = SettlementEvent::default();
  e.kingdom_effects.income = 1;
  s.events.push(e);

  // Done building settlement, add to kingdom
  k.settlements.push(s);

  let mut m = KingdomModifier::default();
  m.effects.income = 1;
  k.modifiers.push(m);

  assert_eq!(
    get_income(&k),
    6,
    "Income is based on Infrastructure, Structures, Events and KingdomModifiers"
  );
}

#[test]
fn consumption() {
  let mut k = Kingdom::default();
  k.infrastructure.claimed_hexes = 10;
  k.infrastructure.farms = 3;
  k.infrastructure.fisheries = 1;

  // Start building settlement
  let mut s = Settlement::default();
  s.districts = 1;

  let mut b = Structure::default();
  b.kingdom_effects.consumption = 1;
  s.structures.push(b);

  let mut e = SettlementEvent::default();
  e.kingdom_effects.consumption = 1;
  s.events.push(e);

  // Done building settlement, add to kingdom
  k.settlements.push(s);

  let mut m = KingdomModifier::default();
  m.effects.consumption = 1;
  k.modifiers.push(m);

  assert_eq!(
    get_consumption(&k),
    7
,
    "Consumption is based on Infrastructure, Districts, Structures, Events and KingdomModifiers"
  );
}
