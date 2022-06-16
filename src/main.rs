use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct KingdomEffects {
  economy: u64,
  loyalty: u64,
  stability: u64,
  fame: u64,
  infamy: u64,
  income: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct SettlementEffects {
  corruption: u64,
  crime: u64,
  law: u64,
  lore: u64,
  productivity: u64,
  society: u64,
  value: u64,
  defence: u64,
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
  name: String,
  settlement_effects: SettlementEffects,
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
  walls: u64,
  structures: Vec<Structure>,
  events: Vec<SettlementEvent>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Courtier {
  name: String,
  bonus: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Court {
  ruler: Option<Courtier>,
  councilor: Option<Courtier>,
  general: Option<Courtier>,
  grand_diplomat: Option<Courtier>,
  high_priest: Option<Courtier>,
  magister: Option<Courtier>,
  marshal: Option<Courtier>,
  royal_enforcer: Option<Courtier>,
  spymaster: Option<Courtier>,
  treasurer: Option<Courtier>,
  viceroy: Option<Courtier>,
  warden: Option<Courtier>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Edicts {
  promotion: u64,
  taxation: u64,
  festivals: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Kingdom {
  name: String,
  claimed_hexes: u64,
  farms: u64,
  fisheries: u64,
  sawmills: u64,
  quarries: u64,
  mines: u64,
  roads: u64,
  rivers: u64,
  edicts: Edicts,
  effects: Vec<KingdomEffects>,
}

fn main() {
  let buf = std::fs::read("./kingdom.yaml").unwrap();
  let kingdom: Kingdom = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&kingdom).unwrap());

  let buf = std::fs::read("./court.yaml").unwrap();
  let court: Court = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&court).unwrap());

  let buf = std::fs::read("./settlements.yaml").unwrap();
  let settlements: Vec<Settlement> = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&settlements).unwrap());
}
