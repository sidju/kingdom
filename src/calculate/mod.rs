use super::*;

mod kingdom;
use kingdom::*;
mod settlement;
use settlement::*;

#[cfg(test)]
mod kingdom_tests;
#[cfg(test)]
mod settlement_tests;

pub fn summarise(k: &Kingdom) -> Summary {
  Summary{
    kingdom: KingdomSummary {
      name: k.name.clone(),
      size: get_size(k),
      control_dc: get_size(k) + 20,
      economy: get_economy(k),
      loyalty: get_loyalty(k),
      stability: get_stability(k),
      unrest: k.unrest,
      consumption: get_consumption(k),
      income: get_income(k),
      treasury: k.treasury,
      fame: get_fame(k),
      infamy: get_infamy(k),
    },
    settlements: k.settlements.iter().map(summarise_settlement).collect(),
  }
}

fn summarise_settlement(s: &Settlement) -> SettlementSummary {
  SettlementSummary{
    name: s.name.clone(),
    districts: s.districts,
    lots: get_lots(&s),
    size: get_settlement_size(&s),
    population: s.population,
    population_estimate: get_lots(&s) * 250,
    base_value: get_value(&s),
    base_limit: get_limit(&s),
    purchase_limit: get_purchase_limit(&s),
    defense: get_defense(&s),
    corruption: get_corruption(&s),
    crime: get_crime(&s),
    law: get_law(&s),
    lore: get_lore(&s),
    productivity: get_productivity(&s),
    society: get_society(&s),
  }
}
