use crate::lcu::match_history::LolMatchHistoryMatchHistoryGame as LcuLolMatchHistoryMatchHistoryGame;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryGameList as LcuLolMatchHistoryMatchHistoryGameList;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryParticipant;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryParticipantIdentities;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryParticipantIdentityPlayer;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryParticipantStatistics;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryTeam;
use crate::lcu::match_history::LolMatchHistoryMatchHistoryTeamBan;
use crate::lcu::match_history::MatchHistory as LcuMatchHistory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchHistory {
    #[serde(rename = "games")]
    pub(crate) games: Option<Vec<Game>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "metadata")]
    pub(crate) metadata: Option<Metadata>,

    #[serde(rename = "json")]
    pub(crate) json: Option<Json>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "product")]
    pub(crate) product: Option<String>,

    #[serde(rename = "tags")]
    pub(crate) tags: Option<Vec<String>>,

    #[serde(rename = "participants")]
    pub(crate) participants: Option<Vec<String>>,

    #[serde(rename = "timestamp")]
    pub(crate) timestamp: Option<String>,

    #[serde(rename = "dataVersion")]
    pub(crate) data_version: Option<String>,

    #[serde(rename = "infoType")]
    pub(crate) info_type: Option<String>,

    #[serde(rename = "matchId")]
    pub(crate) match_id: Option<String>,

    #[serde(rename = "private")]
    pub(crate) private: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Json {
    #[serde(rename = "gameCreation")]
    pub(crate) game_creation: Option<i64>,

    #[serde(rename = "gameDuration")]
    pub(crate) game_duration: Option<i64>,

    #[serde(rename = "gameEndTimestamp")]
    pub(crate) game_end_timestamp: Option<i64>,

    #[serde(rename = "gameId")]
    pub(crate) game_id: Option<i64>,

    #[serde(rename = "gameMode")]
    pub(crate) game_mode: Option<String>,

    #[serde(rename = "gameName")]
    pub(crate) game_name: Option<String>,

    #[serde(rename = "gameStartTimestamp")]
    pub(crate) game_start_timestamp: Option<u64>,

    #[serde(rename = "gameType")]
    pub(crate) game_type: Option<String>,

    #[serde(rename = "gameVersion")]
    pub(crate) game_version: Option<String>,

    #[serde(rename = "mapId")]
    pub(crate) map_id: Option<i64>,

    #[serde(rename = "participants")]
    pub(crate) participants: Option<Vec<Participant>>,

    #[serde(rename = "platformId")]
    pub(crate) platform_id: Option<String>,

    #[serde(rename = "queueId")]
    pub(crate) queue_id: Option<i64>,

    #[serde(rename = "seasonId")]
    pub(crate) season_id: Option<i64>,

    #[serde(rename = "teams")]
    pub(crate) teams: Option<Vec<Team>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "allInPings")]
    pub(crate) all_in_pings: Option<i64>,

    #[serde(rename = "assistMePings")]
    pub(crate) assist_me_pings: Option<i64>,

    #[serde(rename = "assists")]
    pub(crate) assists: Option<i64>,

    #[serde(rename = "baitPings")]
    pub(crate) bait_pings: Option<i64>,

    #[serde(rename = "baronKills")]
    pub(crate) baron_kills: Option<i64>,

    #[serde(rename = "basicPings")]
    pub(crate) basic_pings: Option<i64>,

    #[serde(rename = "bountyLevel")]
    pub(crate) bounty_level: Option<i64>,

    #[serde(rename = "champExperience")]
    pub(crate) champ_experience: Option<i64>,

    #[serde(rename = "champLevel")]
    pub(crate) champ_level: Option<i64>,

    #[serde(rename = "championId")]
    pub(crate) champion_id: Option<i64>,

    #[serde(rename = "championName")]
    pub(crate) champion_name: Option<String>,

    #[serde(rename = "championTransform")]
    pub(crate) champion_transform: Option<i64>,

    #[serde(rename = "commandPings")]
    pub(crate) command_pings: Option<i64>,

    #[serde(rename = "consumablesPurchased")]
    pub(crate) consumables_purchased: Option<i64>,

    #[serde(rename = "damageDealtToBuildings")]
    pub(crate) damage_dealt_to_buildings: Option<i64>,

    #[serde(rename = "damageDealtToObjectives")]
    pub(crate) damage_dealt_to_objectives: Option<i64>,

    #[serde(rename = "damageDealtToTurrets")]
    pub(crate) damage_dealt_to_turrets: Option<i64>,

    #[serde(rename = "damageSelfMitigated")]
    pub(crate) damage_self_mitigated: Option<i64>,

    #[serde(rename = "dangerPings")]
    pub(crate) danger_pings: Option<i64>,

    #[serde(rename = "teams")]
    pub(crate) deaths: Option<i64>,

    #[serde(rename = "detectorWardsPlaced")]
    pub(crate) detector_wards_placed: Option<i64>,

    #[serde(rename = "doubleKills")]
    pub(crate) double_kills: Option<i64>,

    #[serde(rename = "dragonKills")]
    pub(crate) dragon_kills: Option<i64>,

    #[serde(rename = "eligibleForProgression")]
    pub(crate) eligible_for_progression: Option<bool>,

    #[serde(rename = "enemyMissingPings")]
    pub(crate) enemy_missing_pings: Option<i64>,

    #[serde(rename = "enemyVisionPings")]
    pub(crate) enemy_vision_pings: Option<i64>,

    #[serde(rename = "firstBloodAssist")]
    pub(crate) first_blood_assist: Option<bool>,

    #[serde(rename = "firstBloodKill")]
    pub(crate) first_blood_kill: Option<bool>,

    #[serde(rename = "firstTowerAssist")]
    pub(crate) first_tower_assist: Option<bool>,

    #[serde(rename = "firstTowerKill")]
    pub(crate) first_tower_kill: Option<bool>,

    #[serde(rename = "gameEndedInEarlySurrender")]
    pub(crate) game_ended_in_early_surrender: Option<bool>,

    #[serde(rename = "gameEndedInSurrender")]
    pub(crate) game_ended_in_surrender: Option<bool>,

    #[serde(rename = "getBackPings")]
    pub(crate) get_back_pings: Option<i64>,

    #[serde(rename = "goldEarned")]
    pub(crate) gold_earned: Option<i64>,

    #[serde(rename = "goldSpent")]
    pub(crate) gold_spent: Option<i64>,

    #[serde(rename = "holdPings")]
    pub(crate) hold_pings: Option<i64>,

    // individualPosition: Invalid,
    #[serde(rename = "inhibitorKills")]
    pub(crate) inhibitor_kills: Option<i64>,

    #[serde(rename = "inhibitorTakedowns")]
    pub(crate) inhibitor_takedowns: Option<i64>,

    #[serde(rename = "inhibitorsLost")]
    pub(crate) inhibitors_lost: Option<i64>,

    #[serde(rename = "item0")]
    pub(crate) item0: Option<i64>,

    #[serde(rename = "item1")]
    pub(crate) item1: Option<i64>,

    #[serde(rename = "item2")]
    pub(crate) item2: Option<i64>,

    #[serde(rename = "item3")]
    pub(crate) item3: Option<i64>,

    #[serde(rename = "item4")]
    pub(crate) item4: Option<i64>,

    #[serde(rename = "item5")]
    pub(crate) item5: Option<i64>,

    #[serde(rename = "item6")]
    pub(crate) item6: Option<i64>,

    #[serde(rename = "itemsPurchased")]
    pub(crate) items_purchased: Option<i64>,

    #[serde(rename = "killingSprees")]
    pub(crate) killing_sprees: Option<i64>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,

    // lane: NONE,
    #[serde(rename = "largestCriticalStrike")]
    pub(crate) largest_critical_strike: Option<i64>,

    #[serde(rename = "largestKillingSpree")]
    pub(crate) largest_killing_spree: Option<i64>,

    #[serde(rename = "largestMultiKill")]
    pub(crate) largest_multi_kill: Option<i64>,

    #[serde(rename = "longestTimeSpentLiving")]
    pub(crate) longest_time_spent_living: Option<i64>,

    #[serde(rename = "magicDamageDealt")]
    pub(crate) magic_damage_dealt: Option<i64>,

    #[serde(rename = "magicDamageDealtToChampions")]
    pub(crate) magic_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "magicDamageTaken")]
    pub(crate) magic_damage_taken: Option<i64>,

    #[serde(rename = "needVisionPings")]
    pub(crate) need_vision_pings: Option<i64>,

    #[serde(rename = "neutralMinionsKilled")]
    pub(crate) neutral_minions_killed: Option<i64>,

    #[serde(rename = "nexusKills")]
    pub(crate) nexus_kills: Option<i64>,

    #[serde(rename = "nexusLost")]
    pub(crate) nexus_lost: Option<i64>,

    #[serde(rename = "nexusTakedowns")]
    pub(crate) nexus_takedowns: Option<i64>,

    #[serde(rename = "objectivesStolen")]
    pub(crate) objectives_stolen: Option<i64>,

    #[serde(rename = "objectivesStolenAssists")]
    pub(crate) objectives_stolen_assists: Option<i64>,

    #[serde(rename = "onMyWayPings")]
    pub(crate) on_my_way_pings: Option<i64>,

    #[serde(rename = "participantId")]
    pub(crate) participant_id: Option<i64>,

    #[serde(rename = "pentaKills")]
    pub(crate) penta_kills: Option<i64>,

    #[serde(rename = "perks")]
    pub(crate) perks: Option<Perks>,

    #[serde(rename = "physicalDamageDealt")]
    pub(crate) physical_damage_dealt: Option<i64>,

    #[serde(rename = "physicalDamageDealtToChampions")]
    pub(crate) physical_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "physicalDamageTaken")]
    pub(crate) physical_damage_taken: Option<i64>,

    #[serde(rename = "placement")]
    pub(crate) placement: Option<i64>,

    #[serde(rename = "playerAugment1")]
    pub(crate) player_augment1: Option<i64>,

    #[serde(rename = "playerAugment2")]
    pub(crate) player_augment2: Option<i64>,

    #[serde(rename = "playerAugment3")]
    pub(crate) player_augment3: Option<i64>,

    #[serde(rename = "playerAugment4")]
    pub(crate) player_augment4: Option<i64>,

    #[serde(rename = "playerSubteamId")]
    pub(crate) player_subteam_id: Option<i64>,

    #[serde(rename = "profileIcon")]
    pub(crate) profile_icon: Option<i64>,

    #[serde(rename = "pushPings")]
    pub(crate) push_pings: Option<i64>,

    #[serde(rename = "puuid")]
    pub(crate) puuid: Option<String>,

    #[serde(rename = "quadraKills")]
    pub(crate) quadra_kills: Option<i64>,

    // riotIdName: ,

    // riotIdTagline: ,

    // role: DUO,
    #[serde(rename = "sightWardsBoughtInGame")]
    pub(crate) sight_wards_bought_in_game: Option<i64>,

    #[serde(rename = "spell1Casts")]
    pub(crate) spell1_casts: Option<i64>,

    #[serde(rename = "spell1Id")]
    pub(crate) spell1_id: Option<i64>,

    #[serde(rename = "spell2Casts")]
    pub(crate) spell2_casts: Option<i64>,

    #[serde(rename = "spell2Id")]
    pub(crate) spell2_id: Option<i64>,

    #[serde(rename = "spell3Casts")]
    pub(crate) spell3_casts: Option<i64>,

    #[serde(rename = "spell4Casts")]
    pub(crate) spell4_casts: Option<i64>,

    #[serde(rename = "subteamPlacement")]
    pub(crate) subteam_placement: Option<i64>,

    #[serde(rename = "summoner1Casts")]
    pub(crate) summoner1_casts: Option<i64>,

    #[serde(rename = "summoner2Casts")]
    pub(crate) summoner2_casts: Option<i64>,

    #[serde(rename = "summonerId")]
    pub(crate) summoner_id: Option<i64>,

    #[serde(rename = "summonerLevel")]
    pub(crate) summoner_level: Option<i64>,

    #[serde(rename = "summonerName")]
    pub(crate) summoner_name: Option<String>,

    #[serde(rename = "teamEarlySurrendered")]
    pub(crate) team_early_surrendered: Option<bool>,

    #[serde(rename = "teamId")]
    pub(crate) team_id: Option<i64>,

    // teamPosition: ,
    #[serde(rename = "timeCCingOthers")]
    pub(crate) time_ccing_others: Option<i64>,

    #[serde(rename = "timePlayed")]
    pub(crate) time_played: Option<i64>,

    #[serde(rename = "totalAllyJungleMinionsKilled")]
    pub(crate) total_ally_jungle_minions_killed: Option<i64>,

    #[serde(rename = "totalDamageDealt")]
    pub(crate) total_damage_dealt: Option<i64>,

    #[serde(rename = "totalDamageDealtToChampions")]
    pub(crate) total_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "totalDamageShieldedOnTeammates")]
    pub(crate) total_damage_shielded_on_teammates: Option<i64>,

    #[serde(rename = "totalDamageTaken")]
    pub(crate) total_damage_taken: Option<i64>,

    #[serde(rename = "totalEnemyJungleMinionsKilled")]
    pub(crate) total_enemy_jungle_minions_killed: Option<i64>,

    #[serde(rename = "totalHeal")]
    pub(crate) total_heal: Option<i64>,

    #[serde(rename = "totalHealsOnTeammates")]
    pub(crate) total_heals_on_teammates: Option<i64>,

    #[serde(rename = "totalMinionsKilled")]
    pub(crate) total_minions_killed: Option<i64>,

    #[serde(rename = "totalTimeCCDealt")]
    pub(crate) total_time_ccdealt: Option<i64>,

    #[serde(rename = "totalTimeSpentDead")]
    pub(crate) total_time_spent_dead: Option<i64>,

    #[serde(rename = "totalUnitsHealed")]
    pub(crate) total_units_healed: Option<i64>,

    #[serde(rename = "tripleKills")]
    pub(crate) triple_kills: Option<i64>,

    #[serde(rename = "trueDamageDealt")]
    pub(crate) true_damage_dealt: Option<i64>,

    #[serde(rename = "trueDamageDealtToChampions")]
    pub(crate) true_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "trueDamageTaken")]
    pub(crate) true_damage_taken: Option<i64>,

    #[serde(rename = "turretKills")]
    pub(crate) turret_kills: Option<i64>,

    #[serde(rename = "turretTakedowns")]
    pub(crate) turret_takedowns: Option<i64>,

    #[serde(rename = "turretsLost")]
    pub(crate) turrets_lost: Option<i64>,

    #[serde(rename = "unrealKills")]
    pub(crate) unreal_kills: Option<i64>,

    #[serde(rename = "visionClearedPings")]
    pub(crate) vision_cleared_pings: Option<i64>,

    #[serde(rename = "visionScore")]
    pub(crate) vision_score: Option<i64>,

    #[serde(rename = "visionWardsBoughtInGame")]
    pub(crate) vision_wards_bought_in_game: Option<i64>,

    #[serde(rename = "wardsKilled")]
    pub(crate) wards_killed: Option<i64>,

    #[serde(rename = "wardsPlaced")]
    pub(crate) wards_placed: Option<i64>,

    #[serde(rename = "win")]
    pub(crate) win: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "bans")]
    pub(crate) bans: Option<Vec<Ban>>,

    #[serde(rename = "objectives")]
    pub(crate) objectives: Option<Objectives>,

    #[serde(rename = "teamId")]
    pub(crate) team_id: Option<i64>,

    #[serde(rename = "win")]
    pub(crate) win: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ban {
    #[serde(rename = "championId")]
    pub(crate) champion_id: Option<i64>,

    #[serde(rename = "pickTurn")]
    pub(crate) pick_turn: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objectives {
    #[serde(rename = "baron")]
    pub(crate) baron: Option<Baron>,

    #[serde(rename = "champion")]
    pub(crate) champion: Option<Champion>,

    #[serde(rename = "dragon")]
    pub(crate) dragon: Option<Dragon>,

    #[serde(rename = "inhibitor")]
    pub(crate) inhibitor: Option<Inhibitor>,

    #[serde(rename = "riftHerald")]
    pub(crate) rift_herald: Option<RiftHerald>,

    #[serde(rename = "tower")]
    pub(crate) tower: Option<Tower>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Baron {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dragon {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inhibitor {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiftHerald {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tower {
    #[serde(rename = "first")]
    pub(crate) first: Option<bool>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Perks {
    #[serde(rename = "statPerks")]
    pub(crate) stat_perks: Option<StatPerks>,
    #[serde(rename = "styles")]
    pub(crate) styles: Option<Vec<Style>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatPerks {
    #[serde(rename = "defense")]
    pub(crate) defense: Option<i64>,
    #[serde(rename = "flex")]
    pub(crate) flex: Option<i64>,
    #[serde(rename = "offense")]
    pub(crate) offense: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Style {
    #[serde(rename = "description")]
    pub(crate) description: Option<String>,
    #[serde(rename = "selections")]
    pub(crate) selections: Option<Vec<Selection>>,
    #[serde(rename = "style")]
    pub(crate) style: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    #[serde(rename = "perk")]
    pub(crate) perk: Option<i64>,
    #[serde(rename = "var1")]
    pub(crate) var1: Option<i64>,
    #[serde(rename = "var2")]
    pub(crate) var2: Option<i64>,
    #[serde(rename = "var3")]
    pub(crate) var3: Option<i64>,
}

impl Into<LcuMatchHistory> for MatchHistory {
    fn into(self) -> LcuMatchHistory {
        let mut games: Vec<LcuLolMatchHistoryMatchHistoryGame> = Vec::new();
        if self.games.is_some() {
            for g in self.games.as_ref().unwrap() {
                let mut ptcpt: Vec<LolMatchHistoryMatchHistoryParticipant> = Vec::new();
                let mut pi: Vec<LolMatchHistoryMatchHistoryParticipantIdentities> = Vec::new();
                let mut teams: Vec<LolMatchHistoryMatchHistoryTeam> = Vec::new();

                for p in g
                    .json
                    .as_ref()
                    .unwrap()
                    .participants
                    .as_ref()
                    .unwrap()
                    .iter()
                {
                    ptcpt.push(LolMatchHistoryMatchHistoryParticipant {
                        champion_id: p.champion_id,
                        highest_achieved_season_tier: None,
                        participant_id: p.participant_id,
                        spell1_id: p.spell1_id,
                        spell2_id: p.spell2_id,
                        stats: Some(LolMatchHistoryMatchHistoryParticipantStatistics {
                            assists: p.assists,
                            caused_early_surrender: None,
                            champ_level: p.champ_level,
                            combat_player_score: None,
                            damage_dealt_to_objectives: p.damage_dealt_to_objectives,
                            damage_dealt_to_turrets: p.damage_dealt_to_turrets,
                            damage_self_mitigated: p.damage_self_mitigated,
                            deaths: p.deaths,
                            double_kills: p.double_kills,
                            early_surrender_accomplice: None,
                            first_blood_assist: p.first_blood_assist,
                            first_blood_kill: p.first_blood_kill,
                            first_inhibitor_assist: None,
                            first_inhibitor_kill: None,
                            first_tower_assist: p.first_tower_assist,
                            first_tower_kill: p.first_tower_kill,
                            game_ended_in_early_surrender: p.game_ended_in_early_surrender,
                            game_ended_in_surrender: p.game_ended_in_surrender,
                            gold_earned: p.gold_earned,
                            gold_spent: p.gold_spent,
                            inhibitor_kills: p.inhibitor_kills,
                            item0: p.item0,
                            item1: p.item1,
                            item2: p.item2,
                            item3: p.item3,
                            item4: p.item4,
                            item5: p.item5,
                            item6: p.item6,
                            killing_sprees: p.killing_sprees,
                            kills: p.kills,
                            largest_critical_strike: p.largest_critical_strike,
                            largest_killing_spree: p.largest_killing_spree,
                            largest_multi_kill: p.largest_multi_kill,
                            longest_time_spent_living: p.longest_time_spent_living,
                            magical_damage_taken: p.magic_damage_taken,
                            magic_damage_dealt: p.magic_damage_dealt,
                            magic_damage_dealt_to_champions: p.magic_damage_dealt_to_champions,
                            neutral_minions_killed: p.neutral_minions_killed,
                            neutral_minions_killed_enemy_jungle: None,
                            neutral_minions_killed_team_jungle: None,
                            objective_player_score: None,
                            participant_id: p.participant_id,
                            penta_kills: p.penta_kills,
                            perk0: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .perk,
                            perk0_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var1,
                            perk0_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var2,
                            perk0_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var3,
                            perk1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .perk,
                            perk1_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var1,
                            perk1_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var2,
                            perk1_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var3,
                            perk2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[2]
                                .perk,
                            perk2_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[2]
                                .var1,
                            perk2_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[2]
                                .var2,
                            perk2_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[2]
                                .var3,
                            perk3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[3]
                                .perk,
                            perk3_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[3]
                                .var1,
                            perk3_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[3]
                                .var2,
                            perk3_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(0)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[3]
                                .var3,
                            perk4: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .perk,
                            perk4_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var1,
                            perk4_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var2,
                            perk4_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[0]
                                .var3,
                            perk5: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .perk,
                            perk5_var1: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var1,
                            perk5_var2: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var2,
                            perk5_var3: p
                                .perks
                                .as_ref()
                                .unwrap()
                                .styles
                                .as_ref()
                                .unwrap()
                                .get(1)
                                .unwrap()
                                .selections
                                .as_ref()
                                .unwrap()[1]
                                .var3,
                            perk_primary_style: p.perks.as_ref().unwrap().styles.as_ref().unwrap()
                                [0]
                            .style,
                            perk_sub_style: p.perks.as_ref().unwrap().styles.as_ref().unwrap()[1]
                                .style,
                            physical_damage_dealt: p.physical_damage_dealt,
                            physical_damage_dealt_to_champions: p
                                .physical_damage_dealt_to_champions,
                            physical_damage_taken: p.physical_damage_taken,
                            player_score0: None,
                            player_score1: None,
                            player_score2: None,
                            player_score3: None,
                            player_score4: None,
                            player_score5: None,
                            player_score6: None,
                            player_score7: None,
                            player_score8: None,
                            player_score9: None,
                            quadra_kills: p.quadra_kills,
                            sight_wards_bought_in_game: p.sight_wards_bought_in_game,
                            team_early_surrendered: p.team_early_surrendered,
                            time_c_cing_others: p.time_ccing_others,
                            total_damage_dealt: p.total_damage_dealt,
                            total_damage_dealt_to_champions: p.total_damage_dealt_to_champions,
                            total_damage_taken: p.total_damage_taken,
                            total_heal: p.total_heal,
                            total_minions_killed: p.total_minions_killed,
                            total_player_score: None,
                            total_score_rank: None,
                            total_time_crowd_control_dealt: None,
                            total_units_healed: p.total_units_healed,
                            triple_kills: p.triple_kills,
                            true_damage_dealt: p.true_damage_dealt,
                            true_damage_dealt_to_champions: p.true_damage_dealt_to_champions,
                            true_damage_taken: p.true_damage_taken,
                            turret_kills: p.turret_kills,
                            unreal_kills: p.unreal_kills,
                            vision_score: p.vision_score,
                            vision_wards_bought_in_game: p.vision_wards_bought_in_game,
                            wards_killed: p.wards_killed,
                            wards_placed: p.wards_placed,
                            win: p.win,
                        }),
                        team_id: p.team_id,
                        timeline: None,
                    });
                    pi.push(LolMatchHistoryMatchHistoryParticipantIdentities {
                        participant_id: p.participant_id,
                        player: Some(LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
                            account_id: p.summoner_id,
                            current_account_id: p.summoner_id,
                            current_platform_id: None,
                            match_history_uri: None,
                            platform_id: None,
                            profile_icon: p.profile_icon,
                            summoner_id: p.summoner_id,
                            summoner_name: p.summoner_name.clone(),
                        }),
                    });
                }

                for t in g.json.as_ref().unwrap().teams.as_ref().unwrap().iter() {
                    let mut bans: Vec<LolMatchHistoryMatchHistoryTeamBan> = Vec::new();
                    for b in t.bans.as_ref().unwrap().iter() {
                        bans.push(LolMatchHistoryMatchHistoryTeamBan {
                            champion_id: b.champion_id,
                            pick_turn: b.pick_turn,
                        })
                    }
                    teams.push(LolMatchHistoryMatchHistoryTeam {
                        bans: Some(bans),
                        baron_kills: t.objectives.as_ref().unwrap().baron.as_ref().unwrap().kills,
                        dominion_victory_score: None,
                        dragon_kills: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .dragon
                            .as_ref()
                            .unwrap()
                            .kills,
                        first_baron: t.objectives.as_ref().unwrap().baron.as_ref().unwrap().first,
                        first_blood: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .champion
                            .as_ref()
                            .unwrap()
                            .first,
                        first_dargon: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .dragon
                            .as_ref()
                            .unwrap()
                            .first,
                        first_inhibitor: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .inhibitor
                            .as_ref()
                            .unwrap()
                            .first,
                        first_tower: t.objectives.as_ref().unwrap().tower.as_ref().unwrap().first,
                        inhibitor_kills: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .inhibitor
                            .as_ref()
                            .unwrap()
                            .kills,
                        rift_herald_kills: t
                            .objectives
                            .as_ref()
                            .unwrap()
                            .rift_herald
                            .as_ref()
                            .unwrap()
                            .kills,
                        team_id: t.team_id,
                        tower_kills: t.objectives.as_ref().unwrap().tower.as_ref().unwrap().kills,
                        vilemaw_kills: None,
                        win: if t.win.unwrap() {
                            Some(String::from("Win"))
                        } else {
                            Some(String::from("Fail"))
                        },
                    })
                }

                games.push(LcuLolMatchHistoryMatchHistoryGame {
                    game_creation: g.json.as_ref().unwrap().game_creation,
                    game_creation_date: None,
                    game_duration: g.json.as_ref().unwrap().game_duration,
                    game_id: g.json.as_ref().unwrap().game_id,
                    game_mode: g.json.as_ref().unwrap().game_mode.clone(),
                    game_type: g.json.as_ref().unwrap().game_type.clone(),
                    game_version: g.json.as_ref().unwrap().game_version.clone(),
                    map_id: g.json.as_ref().unwrap().map_id,
                    participant_identities: Some(pi),
                    participants: Some(ptcpt),
                    platform_id: g.json.as_ref().unwrap().platform_id.clone(),
                    queue_id: g.json.as_ref().unwrap().queue_id,
                    season_id: g.json.as_ref().unwrap().season_id,
                    teams: Some(teams),
                });
            }
        }

        LcuMatchHistory {
            account_id: None,
            games: Some(LcuLolMatchHistoryMatchHistoryGameList {
                game_begin_date: None,
                game_count: Some(self.games.unwrap().len() as i64),
                game_end_date: None,
                game_index_begin: None,
                game_index_end: None,
                games: Some(games),
            }),
            platform_id: None,
        }
    }
}
