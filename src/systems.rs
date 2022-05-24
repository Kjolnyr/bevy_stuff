use crate::prelude::*;
use std::io::{Write, stdout, stdin};

pub fn input(
    player_name_query: Query<&Name, With<Player>>,
    mut ev_input: EventWriter<InputEvent>
) {
    let player_name = player_name_query.single();

    print!("{} > ", player_name.0);
    stdout().flush().unwrap();
    let mut input_buf = String::new();
    stdin().read_line(&mut input_buf).unwrap();
    let mut args: Vec<String> = input_buf.trim().split(" ").map(|element| element.to_string()).collect();
    let cmd = args.remove(0);

    ev_input.send(InputEvent { cmd, args });

}

pub fn handle_command(
    mut commands: Commands,
    mut player_query: Query<Entity, With<Player>>,
    mut ev_input: EventReader<InputEvent>,
) {
    let player = player_query.single_mut();

    for ev in ev_input.iter() {
        match ev.cmd.as_str() {
            "cc" => { // Create Corp
                let name = ev.args[0].clone();
                let share_amount: u32 = ev.args[1].parse().unwrap();
                let share_price: f64 = ev.args[2].parse().unwrap();

                println!("creating corp {} with {} shares of ${:.2}", name, share_amount, share_price);
                let corp = corporation_entity(
                    &mut commands,
                    player,
                    &name,
                    share_amount,
                    share_price
                );
                commands.entity(corp).insert(PlayerOwned);

            },
            "s" => { // Switch managing entity
                let id: u32 = ev.args[0].parse().unwrap();

                println!("Switching to entity id {}", id);

                let target_entity = Entity::from_raw(id);

                commands.entity(player).remove::<Player>();
                commands.entity(target_entity).insert(Player);
            }
            _ => return
        }
    }
}


pub fn spawn_player(
    mut commands: Commands,
) {
    let player = person_entity(&mut commands, "Kjolnyr", 1_000_000.0);
    commands.entity(player).insert(Player).insert(PlayerOwned);
}


pub fn overview(
    player_person_query: Query<(Entity, &Name, &Cash), (With<PlayerOwned>, With<Person>)>,
    owned_corps_query: Query<(Entity, &Name, &Cash, &Corporation), With<PlayerOwned>> 
) {
    let (player, player_name, player_cash) = player_person_query.single();

    println!("Player {} (id: {}) -> ${:.2}", player_name.0, player.id(), player_cash.0);

    owned_corps_query.iter()
        .for_each(|(entity, name, cash, corpo)| {
            println!("Owned: {} (id: {}) -> ${:.2}\nShare holders: {:#?}", 
            name.0, entity.id(), cash.0, corpo.share_holders);
        })
}