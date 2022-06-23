mod structs;
use structs::*;

fn get_data() -> Kingdom {
  let buf = std::fs::read("./kingdom.yaml").unwrap();
  let mut kingdom: Kingdom = serde_yaml::from_slice(&buf).unwrap();

  let buf = std::fs::read("./court.yaml").unwrap();
  kingdom.court = serde_yaml::from_slice(&buf).unwrap();

  kingdom.settlements = kingdom.settlement_paths.iter().map(| path | {
    let buf = std::fs::read(path).unwrap();
    let settlement: Settlement = serde_yaml::from_slice(&buf).unwrap();
    settlement
  }).collect();

  kingdom
}

fn get_size(data: &Kingdom) -> i64 {
  let mut size = data.infrastructure.claimed_hexes;
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
