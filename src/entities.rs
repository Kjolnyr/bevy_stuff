use crate::prelude::*;

pub fn person_entity(
    commands: &mut Commands,
    person_name: &str,
    person_cash: f64
) -> Entity {

    commands.spawn()
        .insert(Person)
        .insert(Name(person_name.to_string()))
        .insert(Cash(person_cash))
        .insert(OwnedShares(Vec::new()))
        .id()
}

pub fn corporation_entity(
    commands: &mut Commands,
    creator: Entity,
    corpo_name: &str,
    share_amount: u32,
    share_price: f64
) -> Entity {

    commands.spawn()
        .insert(Corporation::new(share_amount, share_price, creator))
        .insert(Name(corpo_name.to_string()))
        .insert(Cash(share_amount as f64 * share_price))
        .insert(OwnedShares(Vec::new()))
        .id()
}