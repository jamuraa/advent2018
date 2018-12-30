use splitmut::{SplitMut, SplitMutError};
use std::{
    cmp::{min, Ordering},
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(PartialEq)]
enum Side {
    Immune,
    Infection,
    Stalemate,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Side::Immune {
            write!(f, "Immune")?;
        } else {
            write!(f, "Infection")?;
        }
        Ok(())
    }
}

struct Army {
    side: Side,
    units: u64,
    hitpoints: u64,
    weak_to: HashSet<String>,
    immune_to: HashSet<String>,
    damage: u64,
    damage_type: String,
    initiative: u64,
}

impl Army {
    fn new(
        side: Side,
        units: u64,
        hitpoints: u64,
        weak_to: &[String],
        immune_to: &[String],
        damage: u64,
        damage_type: String,
        initiative: u64,
    ) -> Army {
        let mut a = Army {
            side,
            units,
            hitpoints,
            damage,
            initiative,
            damage_type: damage_type,
            weak_to: HashSet::new(),
            immune_to: HashSet::new(),
        };
        for weak in weak_to {
            a.weak_to.insert(weak.to_string());
        }
        for immune in immune_to {
            a.immune_to.insert(immune.to_string());
        }
        a
    }

    fn is_side(&self, side: &Side) -> bool {
        &self.side == side
    }

    fn effective_power(&self) -> u64 {
        self.units * self.damage
    }

    fn target_selection_order(&self) -> (u64, u64) {
        (self.effective_power(), self.initiative)
    }

    fn would_damage(&self, other: &Army) -> u64 {
        if other.immune_to.contains(&self.damage_type) {
            return 0;
        }
        if other.weak_to.contains(&self.damage_type) {
            return 2 * self.damage * self.units;
        }
        return self.damage * self.units;
    }

    fn targeting_cmp(&self, a: &Army, b: &Army) -> Ordering {
        if self.would_damage(a) > self.would_damage(b) {
            return Ordering::Less;
        }
        if self.would_damage(b) > self.would_damage(a) {
            return Ordering::Greater;
        }
        if a.effective_power() > b.effective_power() {
            return Ordering::Less;
        }
        if a.effective_power() < b.effective_power() {
            return Ordering::Greater;
        }
        if a.initiative > b.initiative {
            return Ordering::Less;
        }
        if a.initiative < b.initiative {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }

    fn take_damage(&mut self, amt: u64) -> u64 {
        let units_lost = min(amt / self.hitpoints, self.units);
        self.units -= units_lost;
        units_lost
    }
}

impl fmt::Display for Army {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} group has {} units each with {} hit points",
            self.side, self.units, self.hitpoints
        )?;
        if !self.weak_to.is_empty() {
            write!(
                f,
                " weak to {}",
                self.weak_to
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if !self.immune_to.is_empty() {
            write!(
                f,
                " immune to {}",
                self.immune_to
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        write!(
            f,
            " with an attack that does {} {} damage at initiative {}",
            self.damage, self.damage_type, self.initiative
        )?;
        Ok(())
    }
}

fn battle_with_boost(boost_amount: u64) -> Side {
    let mut armies = Vec::new();
    armies.push(Army::new(Side::Immune, 4400, 10384, &["slashing".to_string()], &[], 21 + boost_amount, "radiation".to_string(), 16));
    armies.push(Army::new(Side::Immune, 974, 9326, &["radiation".to_string()], &[], 86 + boost_amount, "cold".to_string(), 19));
    armies.push(Army::new(Side::Immune, 543, 2286, &[], &[], 34 + boost_amount, "cold".to_string(), 13));
    armies.push(Army::new(Side::Immune, 47, 4241, &["slashing".to_string(), "cold".to_string()], &["radiation".to_string()], 889 + boost_amount, "cold".to_string(), 10));
    armies.push(Army::new(Side::Immune, 5986, 4431, &[], &[], 6 + boost_amount, "cold".to_string(), 8));
    armies.push(Army::new(Side::Immune, 688, 1749, &[], &["slashing".to_string(), "radiation".to_string()], 23 + boost_amount, "cold".to_string(), 7));
    armies.push(Army::new(Side::Immune, 61, 1477, &[], &[], 235 + boost_amount, "fire".to_string(), 1));
    armies.push(Army::new(Side::Immune, 505, 9333, &["slashing".to_string(), "cold".to_string()], &[], 174 + boost_amount, "radiation".to_string(), 9));
    armies.push(Army::new(Side::Immune, 3745, 8367, &["cold".to_string()], &["fire".to_string(), "slashing".to_string(), "radiation".to_string()], 21 + boost_amount, "bludgeoning".to_string(), 3));
    armies.push(Army::new(Side::Immune, 111, 3482, &[], &[], 311 + boost_amount, "cold".to_string(), 15));

    armies.push(Army::new(Side::Infection, 2891, 32406, &["fire".to_string(), "bludgeoning".to_string()], &[], 22, "slashing".to_string(), 2));
    armies.push(Army::new(Side::Infection, 1698, 32906, &["radiation".to_string()], &[], 27, "fire".to_string(), 17));
    armies.push(Army::new(Side::Infection, 395, 37715, &[], &["fire".to_string()], 183, "cold".to_string(), 6));
    armies.push(Army::new(Side::Infection, 3560, 45025, &["radiation".to_string()], &["fire".to_string()], 20, "cold".to_string(), 14));
    armies.push(Army::new(Side::Infection, 2335, 15938, &["cold".to_string()], &[], 13, "slashing".to_string(), 11));
    armies.push(Army::new(Side::Infection, 992, 19604, &[], &[ "slashing".to_string(), "bludgeoning".to_string(), "radiation".to_string(), ], 38, "radiation".to_string(), 5));
    armies.push(Army::new(Side::Infection, 5159, 44419, &["fire".to_string()], &["slashing".to_string()], 13, "bludgeoning".to_string(), 4));
    armies.push(Army::new(Side::Infection, 2950, 6764, &["slashing".to_string()], &[], 4, "radiation".to_string(), 18));
    armies.push(Army::new(Side::Infection, 6131, 25384, &["slashing".to_string()], &["bludgeoning".to_string(), "cold".to_string()], 7, "cold".to_string(), 12));
    armies.push(Army::new(Side::Infection, 94, 29265, &["cold".to_string(), "bludgeoning".to_string()], &[], 588, "bludgeoning".to_string(), 20));

    //armies.push(Army::new(Side::Immune, 17, 5390, &["radiation".to_string(), "bludgeoning".to_string()], &[], 4507, "fire".to_string(), 2));
    //armies.push(Army::new(Side::Immune, 989, 1274, &["bludgeoning".to_string(), "slashing".to_string()], &["fire".to_string()], 25, "slashing".to_string(), 3));

    //armies.push(Army::new(Side::Infection, 801, 4706, &["radiation".to_string()], &[], 116, "bludgeoning".to_string(), 1));
    //armies.push(Army::new(Side::Infection, 4485, 2961, &["fire".to_string(), "cold".to_string()], &["radiation".to_string()], 12, "slashing".to_string(), 4));

    let mut round = 1;
    loop {
        //println!("### ROUND {} ###", round);
        // Target selection
        armies.sort_unstable_by(|x, y| y.target_selection_order().cmp(&x.target_selection_order()));
        let units_before = armies.iter().fold(0, |a, x| a + x.units);

        // A map from attackers to attackees, now that we won't reorder them
        let mut attacks = HashMap::new();
        let mut attacked_by = HashMap::new();

        for (idx, army) in armies.iter().enumerate() {
            let mut candidate_target_idx: Vec<usize> = (0..armies.len())
                .filter(|&x| !armies[x].is_side(&army.side))
                .filter(|x| !attacked_by.contains_key(x))
                .collect();
            candidate_target_idx
                .sort_unstable_by(|&a, &b| army.targeting_cmp(&armies[a], &armies[b]));
            if candidate_target_idx.is_empty()
                || army.would_damage(&armies[candidate_target_idx[0]]) == 0
            {
                // No target.
                // println!("Army {} has no valid targets, not attacking.", idx);
                continue;
            }
            attacks.insert(idx, candidate_target_idx[0]);
            attacked_by.insert(candidate_target_idx[0], idx);
        }

        // println!("### Attacking phase ###");
        // Attacking phase.
        let mut attack_order: Vec<usize> = (0..armies.len()).collect();
        attack_order.sort_unstable_by(|&a, &b| armies[b].initiative.cmp(&armies[a].initiative));
        for idx in attack_order {
            if attacks.contains_key(&idx) {
                let (attacking, attacked) = armies.get2_mut(idx, attacks[&idx]);
                let attacking = attacking.unwrap();
                let attacked = attacked.unwrap();
                let damage = attacking.would_damage(&attacked);
                let units_lost = attacked.take_damage(damage);
                //println!("Army {} attacks army {}, doing {} damage and destroying {} units", attacking, attacked, damage, units_lost);
            }
        }

        // Remove armies with no units left
        armies.retain(|x| x.units > 0);

        // If one side has no armies, we are done.
        let (immune, infect): (Vec<&Army>, Vec<&Army>) =
            armies.iter().partition(|x| x.is_side(&Side::Immune));
        if immune.is_empty() || infect.is_empty() {
            break;
        }
        let units_after = armies.iter().fold(0, |a, x| a + x.units);
        if units_before == units_after {
            // No units have died, and we are in a stalemate.
            return Side::Stalemate;
        }
        round += 1;
    }

    println!("These armies are left after the glorious battle:");
    let mut total_units = 0;
    for army in &armies {
        println!("{}", army);
        total_units += army.units;
    }
    println!("There are {} units in the winning army.", total_units);
    armies.remove(0).side
}

fn main() {
    battle_with_boost(0);
    let mut boost = 1;
    println!("Boosting immune system by {}", boost);
    while battle_with_boost(boost) != Side::Immune {
        boost += 1;
        println!("Boosting immune system by {}", boost);
    }
}
