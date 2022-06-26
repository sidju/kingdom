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
