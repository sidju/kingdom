mod structs;
use structs::*;

mod calculate;
use calculate::*;

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

fn main() {
  let data = get_data();

  let size = get_size(&data);
  let control_dc = size + 20;
  let economy = get_economy(&data);
  let loyalty = get_loyalty(&data);
  let stability = get_stability(&data);
  let income = get_income(&data);
  let consumption = get_consumption(&data);
  let fame = get_fame(&data);
  let infamy = get_infamy(&data);

  println!("Size: {size}");
  println!("Control DC: {control_dc}");
  println!("Economy: {economy}");
  println!("Loyalty: {loyalty}");
  println!("Stability: {stability}");
  println!("Income: {income}");
  println!("Consumption: {consumption}");
  println!("Fame: {fame}");
  println!("Infamy: {infamy}");
}
