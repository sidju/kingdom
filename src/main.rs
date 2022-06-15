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
  settlement_effects: SettlementEffects,
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
struct KingdomEvent {
  name: String,
  kingdom_effects: KingdomEffects,
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
  court: Court,
  edicts: Edicts,
  settlements: Vec<Settlement>,
  events: Vec<KingdomEvent>,
}

fn main() {
  let buf = std::fs::read("./kingdom.yaml").unwrap();
  let kingdom: Kingdom = serde_yaml::from_slice(&buf).unwrap();
  println!("{}\n", serde_yaml::to_string(&kingdom).unwrap());
}
