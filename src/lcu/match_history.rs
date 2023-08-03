use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// LolMatchHistoryMatchHistoryList
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchHistory {
    #[serde(rename = "accountId")]
    pub(crate) account_id: Option<i64>,

    #[serde(rename = "games")]
    pub(crate) games: Option<LolMatchHistoryMatchHistoryGameList>,

    #[serde(rename = "platformId")]
    pub(crate) platform_id: Option<String>,
}

/// LolMatchHistoryMatchHistoryGameList
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryGameList {
    #[serde(rename = "gameBeginDate")]
    pub(crate) game_begin_date: Option<String>,

    #[serde(rename = "gameCount")]
    pub(crate) game_count: Option<i64>,

    #[serde(rename = "gameEndDate")]
    pub(crate) game_end_date: Option<String>,

    #[serde(rename = "gameIndexBegin")]
    pub(crate) game_index_begin: Option<i64>,

    #[serde(rename = "gameIndexEnd")]
    pub(crate) game_index_end: Option<i64>,

    #[serde(rename = "games")]
    pub(crate) games: Option<Vec<LolMatchHistoryMatchHistoryGame>>,
}

/// LolMatchHistoryMatchHistoryGame
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryGame {
    #[serde(rename = "gameCreation")]
    pub(crate) game_creation: Option<i64>,

    #[serde(rename = "gameCreationDate")]
    pub(crate) game_creation_date: Option<String>,

    #[serde(rename = "gameDuration")]
    pub(crate) game_duration: Option<i64>,

    #[serde(rename = "gameId")]
    pub(crate) game_id: Option<i64>,

    #[serde(rename = "gameMode")]
    pub(crate) game_mode: Option<String>,

    #[serde(rename = "gameType")]
    pub(crate) game_type: Option<String>,

    #[serde(rename = "gameVersion")]
    pub(crate) game_version: Option<String>,

    #[serde(rename = "mapId")]
    pub(crate) map_id: Option<i64>,

    #[serde(rename = "participantIdentities")]
    pub(crate) participant_identities:
        Option<Vec<LolMatchHistoryMatchHistoryParticipantIdentities>>,

    #[serde(rename = "participants")]
    pub(crate) participants: Option<Vec<LolMatchHistoryMatchHistoryParticipant>>,

    #[serde(rename = "platformId")]
    pub(crate) platform_id: Option<String>,

    #[serde(rename = "queueId")]
    pub(crate) queue_id: Option<i64>,

    #[serde(rename = "seasonId")]
    pub(crate) season_id: Option<i64>,

    #[serde(rename = "teams")]
    pub(crate) teams: Option<Vec<LolMatchHistoryMatchHistoryTeam>>,
}

/// LolMatchHistoryMatchHistoryParticipantIdentities
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    #[serde(rename = "participantId")]
    pub(crate) participant_id: Option<i64>,

    #[serde(rename = "player")]
    pub(crate) player: Option<LolMatchHistoryMatchHistoryParticipantIdentityPlayer>,
}

/// LolMatchHistoryMatchHistoryParticipantIdentityPlayer
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    #[serde(rename = "accountId")]
    pub(crate) account_id: Option<i64>,

    #[serde(rename = "currentAccountId")]
    pub(crate) current_account_id: Option<i64>,

    #[serde(rename = "currentPlatformId")]
    pub(crate) current_platform_id: Option<String>,

    #[serde(rename = "matchHistoryUri")]
    pub(crate) match_history_uri: Option<String>,

    #[serde(rename = "platformId")]
    pub(crate) platform_id: Option<String>,

    #[serde(rename = "profileIcon")]
    pub(crate) profile_icon: Option<i64>,

    #[serde(rename = "summonerId")]
    pub(crate) summoner_id: Option<i64>,

    #[serde(rename = "summonerName")]
    pub(crate) summoner_name: Option<String>,
}

/// LolMatchHistoryMatchHistoryParticipant
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryParticipant {
    #[serde(rename = "championId")]
    pub(crate) champion_id: Option<i64>,

    #[serde(rename = "highestAchievedSeasonTier")]
    pub(crate) highest_achieved_season_tier: Option<String>,

    #[serde(rename = "participantId")]
    pub(crate) participant_id: Option<i64>,

    #[serde(rename = "spell1Id")]
    pub(crate) spell1_id: Option<i64>,

    #[serde(rename = "spell2Id")]
    pub(crate) spell2_id: Option<i64>,

    #[serde(rename = "stats")]
    pub(crate) stats: Option<LolMatchHistoryMatchHistoryParticipantStatistics>,

    #[serde(rename = "teamId")]
    pub(crate) team_id: Option<i64>,

    #[serde(rename = "timeline")]
    pub(crate) timeline: Option<LolMatchHistoryMatchHistoryTimeline>,
}

/// LolMatchHistoryMatchHistoryParticipantStatistics
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    #[serde(rename = "assists")]
    pub(crate) assists: Option<i64>,

    #[serde(rename = "causedEarlySurrender")]
    pub(crate) caused_early_surrender: Option<bool>,

    #[serde(rename = "champLevel")]
    pub(crate) champ_level: Option<i64>,

    #[serde(rename = "combatPlayerScore")]
    pub(crate) combat_player_score: Option<i64>,

    #[serde(rename = "damageDealtToObjectives")]
    pub(crate) damage_dealt_to_objectives: Option<i64>,

    #[serde(rename = "damageDealtToTurrets")]
    pub(crate) damage_dealt_to_turrets: Option<i64>,

    #[serde(rename = "damageSelfMitigated")]
    pub(crate) damage_self_mitigated: Option<i64>,

    #[serde(rename = "deaths")]
    pub(crate) deaths: Option<i64>,

    #[serde(rename = "doubleKills")]
    pub(crate) double_kills: Option<i64>,

    #[serde(rename = "earlySurrenderAccomplice")]
    pub(crate) early_surrender_accomplice: Option<bool>,

    #[serde(rename = "firstBloodAssist")]
    pub(crate) first_blood_assist: Option<bool>,

    #[serde(rename = "firstBloodKill")]
    pub(crate) first_blood_kill: Option<bool>,

    #[serde(rename = "firstInhibitorAssist")]
    pub(crate) first_inhibitor_assist: Option<bool>,

    #[serde(rename = "firstInhibitorKill")]
    pub(crate) first_inhibitor_kill: Option<bool>,

    #[serde(rename = "firstTowerAssist")]
    pub(crate) first_tower_assist: Option<bool>,

    #[serde(rename = "firstTowerKill")]
    pub(crate) first_tower_kill: Option<bool>,

    #[serde(rename = "gameEndedInEarlySurrender")]
    pub(crate) game_ended_in_early_surrender: Option<bool>,

    #[serde(rename = "gameEndedInSurrender")]
    pub(crate) game_ended_in_surrender: Option<bool>,

    #[serde(rename = "goldEarned")]
    pub(crate) gold_earned: Option<i64>,

    #[serde(rename = "goldSpent")]
    pub(crate) gold_spent: Option<i64>,

    #[serde(rename = "inhibitorKills")]
    pub(crate) inhibitor_kills: Option<i64>,

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

    #[serde(rename = "killingSprees")]
    pub(crate) killing_sprees: Option<i64>,

    #[serde(rename = "kills")]
    pub(crate) kills: Option<i64>,

    #[serde(rename = "largestCriticalStrike")]
    pub(crate) largest_critical_strike: Option<i64>,

    #[serde(rename = "largestKillingSpree")]
    pub(crate) largest_killing_spree: Option<i64>,

    #[serde(rename = "largestMultiKill")]
    pub(crate) largest_multi_kill: Option<i64>,

    #[serde(rename = "longestTimeSpentLiving")]
    pub(crate) longest_time_spent_living: Option<i64>,

    #[serde(rename = "magicalDamageTaken")]
    pub(crate) magical_damage_taken: Option<i64>,

    #[serde(rename = "magicDamageDealt")]
    pub(crate) magic_damage_dealt: Option<i64>,

    #[serde(rename = "magicDamageDealtToChampions")]
    pub(crate) magic_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "neutralMinionsKilled")]
    pub(crate) neutral_minions_killed: Option<i64>,

    #[serde(rename = "neutralMinionsKilledEnemyJungle")]
    pub(crate) neutral_minions_killed_enemy_jungle: Option<i64>,

    #[serde(rename = "neutralMinionsKilledTeamJungle")]
    pub(crate) neutral_minions_killed_team_jungle: Option<i64>,

    #[serde(rename = "objectivePlayerScore")]
    pub(crate) objective_player_score: Option<i64>,

    #[serde(rename = "participantId")]
    pub(crate) participant_id: Option<i64>,

    #[serde(rename = "pentaKills")]
    pub(crate) penta_kills: Option<i64>,

    #[serde(rename = "perk0")]
    pub(crate) perk0: Option<i64>,

    #[serde(rename = "perk0Var1")]
    pub(crate) perk0_var1: Option<i64>,

    #[serde(rename = "perk0Var2")]
    pub(crate) perk0_var2: Option<i64>,

    #[serde(rename = "perk0Var3")]
    pub(crate) perk0_var3: Option<i64>,

    #[serde(rename = "perk1")]
    pub(crate) perk1: Option<i64>,

    #[serde(rename = "perk1Var1")]
    pub(crate) perk1_var1: Option<i64>,

    #[serde(rename = "perk1Var2")]
    pub(crate) perk1_var2: Option<i64>,

    #[serde(rename = "perk1Var3")]
    pub(crate) perk1_var3: Option<i64>,

    #[serde(rename = "perk2")]
    pub(crate) perk2: Option<i64>,

    #[serde(rename = "perk2Var1")]
    pub(crate) perk2_var1: Option<i64>,

    #[serde(rename = "perk2Var2")]
    pub(crate) perk2_var2: Option<i64>,

    #[serde(rename = "perk2Var3")]
    pub(crate) perk2_var3: Option<i64>,

    #[serde(rename = "perk3")]
    pub(crate) perk3: Option<i64>,

    #[serde(rename = "perk3Var1")]
    pub(crate) perk3_var1: Option<i64>,

    #[serde(rename = "perk3Var2")]
    pub(crate) perk3_var2: Option<i64>,

    #[serde(rename = "perk3Var3")]
    pub(crate) perk3_var3: Option<i64>,

    #[serde(rename = "perk4")]
    pub(crate) perk4: Option<i64>,

    #[serde(rename = "perk4Var1")]
    pub(crate) perk4_var1: Option<i64>,

    #[serde(rename = "perk4Var2")]
    pub(crate) perk4_var2: Option<i64>,

    #[serde(rename = "perk4Var3")]
    pub(crate) perk4_var3: Option<i64>,

    #[serde(rename = "perk5")]
    pub(crate) perk5: Option<i64>,

    #[serde(rename = "perk5Var1")]
    pub(crate) perk5_var1: Option<i64>,

    #[serde(rename = "perk5Var2")]
    pub(crate) perk5_var2: Option<i64>,

    #[serde(rename = "perk5Var3")]
    pub(crate) perk5_var3: Option<i64>,

    #[serde(rename = "perkPrimaryStyle")]
    pub(crate) perk_primary_style: Option<i64>,

    #[serde(rename = "perkSubStyle")]
    pub(crate) perk_sub_style: Option<i64>,

    #[serde(rename = "physicalDamageDealt")]
    pub(crate) physical_damage_dealt: Option<i64>,

    #[serde(rename = "physicalDamageDealtToChampions")]
    pub(crate) physical_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "physicalDamageTaken")]
    pub(crate) physical_damage_taken: Option<i64>,

    #[serde(rename = "playerScore0")]
    pub(crate) player_score0: Option<i64>,

    #[serde(rename = "playerScore1")]
    pub(crate) player_score1: Option<i64>,

    #[serde(rename = "playerScore2")]
    pub(crate) player_score2: Option<i64>,

    #[serde(rename = "playerScore3")]
    pub(crate) player_score3: Option<i64>,

    #[serde(rename = "playerScore4")]
    pub(crate) player_score4: Option<i64>,

    #[serde(rename = "playerScore5")]
    pub(crate) player_score5: Option<i64>,

    #[serde(rename = "playerScore6")]
    pub(crate) player_score6: Option<i64>,

    #[serde(rename = "playerScore7")]
    pub(crate) player_score7: Option<i64>,

    #[serde(rename = "playerScore8")]
    pub(crate) player_score8: Option<i64>,

    #[serde(rename = "playerScore9")]
    pub(crate) player_score9: Option<i64>,

    #[serde(rename = "quadraKills")]
    pub(crate) quadra_kills: Option<i64>,

    #[serde(rename = "sightWardsBoughtInGame")]
    pub(crate) sight_wards_bought_in_game: Option<i64>,

    #[serde(rename = "teamEarlySurrendered")]
    pub(crate) team_early_surrendered: Option<bool>,

    #[serde(rename = "timeCCingOthers")]
    pub(crate) time_c_cing_others: Option<i64>,

    #[serde(rename = "totalDamageDealt")]
    pub(crate) total_damage_dealt: Option<i64>,

    #[serde(rename = "totalDamageDealtToChampions")]
    pub(crate) total_damage_dealt_to_champions: Option<i64>,

    #[serde(rename = "totalDamageTaken")]
    pub(crate) total_damage_taken: Option<i64>,

    #[serde(rename = "totalHeal")]
    pub(crate) total_heal: Option<i64>,

    #[serde(rename = "totalMinionsKilled")]
    pub(crate) total_minions_killed: Option<i64>,

    #[serde(rename = "totalPlayerScore")]
    pub(crate) total_player_score: Option<i64>,

    #[serde(rename = "totalScoreRank")]
    pub(crate) total_score_rank: Option<i64>,

    #[serde(rename = "totalTimeCrowdControlDealt")]
    pub(crate) total_time_crowd_control_dealt: Option<i64>,

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

    #[serde(rename = "unrealKills")]
    pub(crate) unreal_kills: Option<i64>,

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

/// LolMatchHistoryMatchHistoryTimeline
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryTimeline {
    #[serde(rename = "creepsPerMinDeltas")]
    pub(crate) creeps_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "csDiffPerMinDeltas")]
    pub(crate) cs_diff_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "damageTakenDiffPerMinDeltas")]
    pub(crate) damage_taken_diff_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "damageTakenPerMinDeltas")]
    pub(crate) damage_taken_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "goldPerMinDeltas")]
    pub(crate) gold_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "lane")]
    pub(crate) lane: Option<String>,

    #[serde(rename = "participantId")]
    pub(crate) participant_id: Option<i64>,

    #[serde(rename = "role")]
    pub(crate) role: Option<String>,

    #[serde(rename = "xpDiffPerMinDeltas")]
    pub(crate) xp_diff_per_min_deltas: Option<HashMap<String, f64>>,

    #[serde(rename = "xpPerMinDeltas")]
    pub(crate) xp_per_min_deltas: Option<HashMap<String, f64>>,
}

/// LolMatchHistoryMatchHistoryTeam
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryTeam {
    #[serde(rename = "bans")]
    pub(crate) bans: Option<Vec<LolMatchHistoryMatchHistoryTeamBan>>,

    #[serde(rename = "baronKills")]
    pub(crate) baron_kills: Option<i64>,

    #[serde(rename = "dominionVictoryScore")]
    pub(crate) dominion_victory_score: Option<i64>,

    #[serde(rename = "dragonKills")]
    pub(crate) dragon_kills: Option<i64>,

    #[serde(rename = "firstBaron")]
    pub(crate) first_baron: Option<bool>,

    #[serde(rename = "firstBlood")]
    pub(crate) first_blood: Option<bool>,

    #[serde(rename = "firstDargon")]
    pub(crate) first_dargon: Option<bool>,

    #[serde(rename = "firstInhibitor")]
    pub(crate) first_inhibitor: Option<bool>,

    #[serde(rename = "firstTower")]
    pub(crate) first_tower: Option<bool>,

    #[serde(rename = "inhibitorKills")]
    pub(crate) inhibitor_kills: Option<i64>,

    #[serde(rename = "riftHeraldKills")]
    pub(crate) rift_herald_kills: Option<i64>,

    #[serde(rename = "teamId")]
    pub(crate) team_id: Option<i64>,

    #[serde(rename = "towerKills")]
    pub(crate) tower_kills: Option<i64>,

    #[serde(rename = "vilemawKills")]
    pub(crate) vilemaw_kills: Option<i64>,

    #[serde(rename = "win")]
    pub(crate) win: Option<String>,
}

/// LolMatchHistoryMatchHistoryTeamBan
#[derive(Debug, Serialize, Deserialize)]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    #[serde(rename = "championId")]
    pub(crate) champion_id: Option<i64>,

    #[serde(rename = "pickTurn")]
    pub(crate) pick_turn: Option<i64>,
}
