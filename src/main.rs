use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct KingdomEffects {
  economy: i64,
  loyalty: i64,
  stability: i64,
  fame: i64,
  infamy: i64,
  income: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct SettlementEffects {
  corruption: i64,
  crime: i64,
  law: i64,
  lore: i64,
  productivity: i64,
  society: i64,
  value: i64,
  defence: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Structure {
  name: String,
  lots: u8,
  #[serde(flatten)]
  settlement_effects: SettlementEffects,
  #[serde(flatten)]
  kingdom_effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct SettlementEvent {
  description: String,
  #[serde(flatten)]
  settlement_effects: SettlementEffects,
  #[serde(flatten)]
  kingdom_effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct KingdomModifier {
  cause: String,
  effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Settlement {
  name: String,
  walls: i64,
  structures: Vec<Structure>,
  events: Vec<SettlementEvent>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Courtier {
  name: String,
  bonus: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Court {
  ruler: Option<Courtier>,
  consort: Option<Courtier>,
  heir: Option<Courtier>,
  councilor: Option<Courtier>,
  general: Option<Courtier>,
  grand_diplomat: Option<Courtier>,
  high_priest: Option<Courtier>,
  magister: Option<Courtier>,
  marshal: Option<Courtier>,
  #[serde(alias = "royalExecutioner")]
  royal_enforcer: Option<Courtier>,
  #[serde(alias = "minister")]
  spymaster: Option<Courtier>,
  treasurer: Option<Courtier>,
  viceroy: Option<Courtier>,
  warden: Option<Courtier>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Edicts {
  promotion: i64,
  taxation: i64,
  festivals: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Kingdom {
  name: String,
  claimed_hexes: i64,
  farms: i64,
  fisheries: i64,
  sawmills: i64,
  quarries: i64,
  mines: i64,
  roads: i64,
  rivers: i64,
  edicts: Edicts,
  modifiers: Vec<KingdomEffects>,
  #[serde(alias = "settlements")]
  settlement_paths: Vec<String>,
}

fn main() {
  let buf = std::fs::read("./kingdom.yaml").unwrap();
  let kingdom: Kingdom = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&kingdom).unwrap());

  let buf = std::fs::read("./court.yaml").unwrap();
  let court: Court = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&court).unwrap());

  let settlements: Vec<Settlement> = kingdom.settlement_paths.iter().map(| path | {
    let buf = std::fs::read(path).unwrap();
    let settlement: Settlement = serde_yaml::from_slice(&buf).unwrap();
    settlement
  }).collect();
  println!("{}\n", serde_yaml::to_string(&settlements).unwrap());
}
