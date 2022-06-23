mod structs;
use structs::*;

fn get_data() -> Wrapper {
  let buf = std::fs::read("./kingdom.yaml").unwrap();
  let kingdom: Kingdom = serde_yaml::from_slice(&buf).unwrap();

  let buf = std::fs::read("./court.yaml").unwrap();
  let court: Vec<Courtier> = serde_yaml::from_slice(&buf).unwrap();

  let settlements: Vec<Settlement> = kingdom.settlement_paths.iter().map(| path | {
    let buf = std::fs::read(path).unwrap();
    let settlement: Settlement = serde_yaml::from_slice(&buf).unwrap();
    settlement
  }).collect();

  Wrapper{
    kingdom,
    court,
    settlements,
  }
}

fn get_size(data: &Wrapper) -> i64 {
  let mut size = data.kingdom.infrastructure.claimed_hexes;
  for s in &data.settlements {
    size += s.districts;
  }
  size
}

fn main() {
  let data = get_data();

  let size = get_size(&data);

  println!("{}", size);
}
