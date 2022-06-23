use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Edicts {
  pub promotion: i64,
  pub taxation: i64,
  pub festivals: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CourtierBonus {
  pub economy: i64,
  pub loyalty: i64,
  pub stability: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Courtier {
  pub name: String,
  pub role: String,
  pub bonus: CourtierBonus,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KingdomModifier {
  pub cause: String,
  pub effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct KingdomEffects {
  pub economy: i64,
  pub loyalty: i64,
  pub stability: i64,
  pub fame: i64,
  pub infamy: i64,
  pub income: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SettlementEffects {
  pub corruption: i64,
  pub crime: i64,
  pub law: i64,
  pub lore: i64,
  pub productivity: i64,
  pub society: i64,
  pub value: i64,
  pub defence: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Structure {
  pub name: String,
  pub lots: u8,
  #[serde(flatten)]
  pub settlement_effects: SettlementEffects,
  #[serde(flatten)]
  pub kingdom_effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SettlementEvent {
  pub description: String,
  #[serde(flatten)]
  pub settlement_effects: SettlementEffects,
  #[serde(flatten)]
  pub kingdom_effects: KingdomEffects,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settlement {
  pub name: String,
  pub districts: i64,
  pub walls: i64,
  pub structures: Vec<Structure>,
  pub events: Vec<SettlementEvent>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Kingdom {
  pub name: String,
  pub infrastructure: Infrastructure,
  pub edicts: Edicts,
  pub modifiers: Vec<KingdomEffects>,
  #[serde(alias = "settlements")]
  pub settlement_paths: Vec<String>,
}

pub struct Wrapper {
  pub kingdom: Kingdom,
  pub court: Vec<Courtier>,
  pub settlements: Vec<Settlement>,
}
