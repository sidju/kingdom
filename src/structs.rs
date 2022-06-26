use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Infrastructure {
  pub claimed_hexes: i64,
  pub farms: i64,
  pub fisheries: i64,
  pub sawmills: i64,
  pub quarries: i64,
  pub mines: i64,
  pub roads: i64,
  pub rivers: i64,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CourtierBonus {
  pub economy: i64,
  pub loyalty: i64,
  pub stability: i64,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Courtier {
  pub name: String,
  pub role: String,
  pub bonus: CourtierBonus,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KingdomModifier {
  pub cause: String,
  pub effects: KingdomEffects,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KingdomEffects {
  pub economy: i64,
  pub loyalty: i64,
  pub stability: i64,
  pub fame: i64,
  pub infamy: i64,
  pub income: i64,
  pub consumption: i64,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SettlementEffects {
  pub corruption: i64,
  pub crime: i64,
  pub law: i64,
  pub lore: i64,
  pub productivity: i64,
  pub society: i64,
  pub value: i64,
  pub defense: i64,
}

fn one() -> i64 { 1 }

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Structure {
  pub name: String,
  #[serde(default = "one")]
  pub lots: i64,
  #[serde(flatten)]
  pub s_effects: SettlementEffects,
  #[serde(flatten)]
  pub k_effects: KingdomEffects,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Event {
  pub description: String,
  #[serde(flatten)]
  pub s_effects: SettlementEffects,
  #[serde(flatten)]
  pub k_effects: KingdomEffects,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settlement {
  pub name: String,
  pub districts: i64,
  pub walls: i64,
  pub structures: Vec<Structure>,
  pub events: Vec<Event>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Kingdom {
  pub name: String,
  pub unrest: i64,
  pub treasury: i64,
  pub infrastructure: Infrastructure,
  pub court: Vec<Courtier>,
  pub settlements: Vec<Settlement>,
  pub modifiers: Vec<KingdomModifier>,
  #[serde(alias = "settlements")]
  pub settlement_paths: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementSummary {
  pub name: String,
  pub districts: i64,
  pub lots: i64,
  pub population: i64,
  pub value: i64,
  pub defense: i64,
  pub corruption: i64,
  pub crime: i64,
  pub law: i64,
  pub lore: i64,
  pub productivity: i64,
  pub society: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KingdomSummary {
  pub name: String,
  pub size: i64,
  pub control_dc: i64,
  pub economy: i64,
  pub loyalty: i64,
  pub stability: i64,
  pub unrest: i64,
  pub consumption: i64,
  pub income: i64,
  pub treasury: i64,
  pub fame: i64,
  pub infamy: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
  pub settlements: Vec<SettlementSummary>,
  pub kingdom: KingdomSummary,
}
