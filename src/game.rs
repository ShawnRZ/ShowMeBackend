use serde::{Deserialize, Serialize};

// use crate::lcu;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    creation: Option<u64>,
    duration: Option<u32>,
    id: Option<u64>,
    mode: Option<String>,
    g_type: Option<String>,
    version: Option<String>,
    map_id: Option<String>,
    platform_id: Option<String>,
    queue_id: Option<u32>,
    participants: Vec<Participant>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Participant {
    champion_id: Option<i32>,
    participant_id: Option<i32>,
    spell1_id: Option<i32>,
    spell2_id: Option<i32>,
    stats: Option<Stats>,
    team_id: Option<i32>,
    account_id: Option<u64>,
    puuid: Option<String>,
    summonesr_id: Option<u64>,
    summoner_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    assists: Option<i32>,
    caused_early_surrender: Option<bool>,
    champ_level: Option<i32>,
    combat_player_score: Option<i32>,
    damage_dealt_to_objectives: Option<i32>,
    damage_dealt_to_turrets: Option<i32>,
    damage_self_mitigated: Option<i32>,
    deaths: Option<i32>,
    double_kills: Option<i32>,
    early_surrender_accomplice: Option<bool>,
    first_blood_assist: Option<bool>,
    first_blood_kill: Option<bool>,
    first_inhibitor_assist: Option<bool>,
    first_inhibitor_kill: Option<bool>,
    first_tower_assist: Option<bool>,
    first_tower_kill: Option<bool>,
    game_ended_in_early_surrender: Option<bool>,
    game_ended_in_surrender: Option<bool>,
    gold_earned: Option<i32>,
    gold_spent: Option<i32>,
    inhibitor_kills: Option<i32>,
    item0: Option<i32>,
    item1: Option<i32>,
    item2: Option<i32>,
    item3: Option<i32>,
    item4: Option<i32>,
    item5: Option<i32>,
    item6: Option<i32>,
    killing_sprees: Option<i32>,
    kills: Option<i32>,
    largest_critical_strike: Option<i32>,
    largest_killing_spree: Option<i32>,
    largest_multi_kill: Option<i32>,
    longest_time_spent_living: Option<i32>,
    magic_damage_dealt: Option<i32>,
    magic_damage_dealt_to_champions: Option<i32>,
    magical_damage_taken: Option<i32>,
    neutral_minions_killed: Option<i32>,
    neutral_minions_killed_enemy_jungle: Option<i32>,
    neutral_minions_killed_team_jungle: Option<i32>,
    objective_player_score: Option<i32>,
    participant_id: Option<i32>,
    penta_kills: Option<i32>,
    perk0: Option<i32>,
    perk0_var1: Option<i32>,
    perk0_var2: Option<i32>,
    perk0_var3: Option<i32>,
    perk1: Option<i32>,
    perk1_var1: Option<i32>,
    perk1_var2: Option<i32>,
    perk1_var3: Option<i32>,
    perk2: Option<i32>,
    perk2_var1: Option<i32>,
    perk2_var2: Option<i32>,
    perk2_var3: Option<i32>,
    perk3: Option<i32>,
    perk3_var1: Option<i32>,
    perk3_var2: Option<i32>,
    perk3_var3: Option<i32>,
    perk4: Option<i32>,
    perk4_var1: Option<i32>,
    perk4_var2: Option<i32>,
    perk4_var3: Option<i32>,
    perk5: Option<i32>,
    perk5_var1: Option<i32>,
    perk5_var2: Option<i32>,
    perk5_var3: Option<i32>,
    perk_primary_style: Option<i32>,
    perk_sub_style: Option<i32>,
    physical_damage_dealt: Option<i32>,
    physical_damage_dealt_to_champions: Option<i32>,
    physical_damage_taken: Option<i32>,
    player_score0: Option<i32>,
    player_score1: Option<i32>,
    player_score2: Option<i32>,
    player_score3: Option<i32>,
    player_score4: Option<i32>,
    player_score5: Option<i32>,
    player_score6: Option<i32>,
    player_score7: Option<i32>,
    player_score8: Option<i32>,
    player_score9: Option<i32>,
    quadra_kills: Option<i32>,
    sight_wards_bought_in_game: Option<i32>,
    team_early_surrendered: Option<bool>,
    time_ccing_others: Option<i32>,
    total_damage_dealt: Option<i32>,
    total_damage_dealt_to_champions: Option<i32>,
    total_damage_taken: Option<i32>,
    total_heal: Option<i32>,
    total_minions_killed: Option<i32>,
    total_player_score: Option<i32>,
    total_score_rank: Option<i32>,
    total_time_crowd_control_dealt: Option<i32>,
    total_units_healed: Option<i32>,
    triple_kills: Option<i32>,
    true_damage_dealt: Option<i32>,
    true_damage_dealt_to_champions: Option<i32>,
    true_damage_taken: Option<i32>,
    turret_kills: Option<i32>,
    unreal_kills: Option<i32>,
    vision_score: Option<i32>,
    vision_wards_bought_in_game: Option<i32>,
    wards_killed: Option<i32>,
    wards_placed: Option<i32>,
    win: Option<bool>,
}

// type Error = Box<dyn std::error::Error + Send + Sync>;

// impl Game {
//     pub async fn query_from_summoner(
//         puuid: String,
//         beg: u64,
//         end: u64,
//     ) -> Result<Vec<Game>, Error> {
//         let res = lcu::request::get_match_history(&puuid, beg, end).await?;
//         let m: Vec<Game> = serde_json::from_str(&res)?;
//         Ok(m)
//     }
// }

#[cfg(test)]
mod test {
    // use crate::lcu::refresh_parameter;

    // use super::Game;

    // #[test]
    // fn query_from_summoner() {
    //     tokio_test::block_on(refresh_parameter()).unwrap();
    //     let m = tokio_test::block_on(Game::query_from_summoner(
    //         "55e4504d-b52e-526f-ac89-ee1eeebd842b".to_string(),
    //         0,
    //         1,
    //     ))
    //     .unwrap();
    //     println!("{:#?}", m);
    // }
}
