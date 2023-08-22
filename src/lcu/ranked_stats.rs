use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LolRankedRankedStats
#[derive(Debug, Serialize, Deserialize)]
pub struct RankedStats {
    #[serde(rename = "earnedRegaliaRewardIds")]
    earned_regalia_reward_ids: Option<Vec<String>>,

    #[serde(rename = "highestPreviousSeasonAchievedDivision")]
    highest_previous_season_achieved_division: Option<LolRankedLeagueDivision>,

    #[serde(rename = "highestPreviousSeasonAchievedTier")]
    highest_previous_season_achieved_tier: Option<String>,

    #[serde(rename = "highestPreviousSeasonEndDivision")]
    highest_previous_season_end_division: Option<LolRankedLeagueDivision>,

    #[serde(rename = "highestPreviousSeasonEndTier")]
    highest_previous_season_end_tier: Option<String>,

    #[serde(rename = "highestRankedEntry")]
    highest_ranked_entry: Option<LolRankedRankedQueueStats>,

    #[serde(rename = "highestRankedEntrySR")]
    highest_ranked_entry_sr: Option<LolRankedRankedQueueStats>,

    #[serde(rename = "queueMap")]
    queue_map: Option<HashMap<String, LolRankedRankedQueueStats>>,

    #[serde(rename = "queues")]
    queues: Option<Vec<LolRankedRankedQueueStats>>,

    #[serde(rename = "rankedRegaliaLevel")]
    ranked_regalia_level: Option<i64>,

    #[serde(rename = "seasons")]
    seasons: Option<HashMap<String, LolRankedSeasonDto>>,

    #[serde(rename = "splitsProgress")]
    splits_progress: Option<HashMap<String, i64>>,
}

/// LolRankedRankedQueueStats
#[derive(Debug, Serialize, Deserialize)]
pub struct LolRankedRankedQueueStats {
    #[serde(rename = "division")]
    division: Option<LolRankedLeagueDivision>,

    #[serde(rename = "isProvisional")]
    is_provisional: Option<bool>,

    #[serde(rename = "leaguePoints")]
    league_points: Option<i64>,

    #[serde(rename = "losses")]
    losses: Option<i64>,

    #[serde(rename = "miniSeriesProgress")]
    mini_series_progress: Option<String>,

    #[serde(rename = "previousSeasonAchievedDivision")]
    previous_season_achieved_division: Option<LolRankedLeagueDivision>,

    #[serde(rename = "previousSeasonAchievedTier")]
    previous_season_achieved_tier: Option<String>,

    #[serde(rename = "previousSeasonEndDivision")]
    previous_season_end_division: Option<LolRankedLeagueDivision>,

    #[serde(rename = "previousSeasonEndTier")]
    previous_season_end_tier: Option<String>,

    #[serde(rename = "provisionalGamesRemaining")]
    provisional_games_remaining: Option<i64>,

    #[serde(rename = "provisionalGameThreshold")]
    provisional_game_threshold: Option<i64>,

    #[serde(rename = "queueType")]
    queue_type: Option<String>,

    #[serde(rename = "ratedRating")]
    rated_rating: Option<i64>,

    #[serde(rename = "ratedTier")]
    rated_tier: Option<LolRankedRatedTier>,

    #[serde(rename = "tier")]
    tier: Option<String>,

    // #[serde(rename = "warnings")]
    // warnings: Option<LolRankedRankedQueueWarnings>,

    #[serde(rename = "wins")]
    wins: Option<i64>,
}

// /// LolRankedRankedQueueWarnings
// #[derive(Debug, Serialize, Deserialize)]
// pub struct LolRankedRankedQueueWarnings {
//     #[serde(rename = "daysUntilDecay")]
//     days_until_decay: Option<i64>,

//     #[serde(rename = "demotionWarning")]
//     demotion_warning: Option<i64>,

//     #[serde(rename = "displayDecayWarning")]
//     display_decay_warning: Option<bool>,

//     #[serde(rename = "timeUntilInactivityStatusChanges")]
//     time_until_inactivity_status_changes: Option<i64>,
// }

/// LolRankedSeasonDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct LolRankedSeasonDto {
    #[serde(rename = "currentSeasonEnd")]
    current_season_end: Option<i64>,

    #[serde(rename = "currentSeasonId")]
    current_season_id: Option<i64>,

    #[serde(rename = "nextSeasonStart")]
    next_season_start: Option<i64>,
}

/// LolRankedLeagueDivision
#[derive(Debug, Serialize, Deserialize)]
pub enum LolRankedLeagueDivision {
    #[serde(rename = "I")]
    I,

    #[serde(rename = "II")]
    Ii,

    #[serde(rename = "III")]
    Iii,

    #[serde(rename = "IV")]
    Iv,

    #[serde(rename = "NA")]
    Na,

    #[serde(rename = "V")]
    V,
}

/// LolRankedLeagueTier
// #[derive(Debug, Serialize, Deserialize)]
// pub enum LolRankedLeagueTier {
//     #[serde(rename = "BRONZE")]
//     Bronze,

//     #[serde(rename = "CHALLENGER")]
//     Challenger,

//     #[serde(rename = "DIAMOND")]
//     Diamond,

//     #[serde(rename = "GOLD")]
//     Gold,

//     #[serde(rename = "GRANDMASTER")]
//     Grandmaster,

//     #[serde(rename = "IRON")]
//     Iron,

//     #[serde(rename = "MASTER")]
//     Master,

//     #[serde(rename = "NONE")]
//     #[serde(alias = "")]
//     None,

//     #[serde(rename = "PLATINUM")]
//     Platinum,

//     #[serde(rename = "SILVER")]
//     Silver,
// }

/// LolRankedLeagueQueueType
// #[derive(Debug, Serialize, Deserialize)]
// pub enum LolRankedLeagueQueueType {
//     #[serde(rename = "NONE")]
//     None,

//     #[serde(rename = "RANKED_FLEX_SR")]
//     RankedFlexSr,

//     #[serde(rename = "RANKED_FLEX_TT")]
//     RankedFlexTt,

//     #[serde(rename = "RANKED_SOLO_5x5")]
//     RankedSolo5X5,

//     #[serde(rename = "RANKED_TFT")]
//     RankedTft,

//     #[serde(rename = "RANKED_TFT_PAIRS")]
//     RankedTftPairs,

//     #[serde(rename = "RANKED_TFT_TURBO")]
//     RankedTftTurbo,

//     #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
//     RankedTftDoubleUp,
// }

/// LolRankedRatedTier
#[derive(Debug, Serialize, Deserialize)]
pub enum LolRankedRatedTier {
    #[serde(rename = "BLUE")]
    Blue,

    #[serde(rename = "GRAY")]
    Gray,

    #[serde(rename = "GREEN")]
    Green,

    #[serde(rename = "NONE")]
    None,

    #[serde(rename = "ORANGE")]
    Orange,

    #[serde(rename = "PURPLE")]
    Purple,
}
