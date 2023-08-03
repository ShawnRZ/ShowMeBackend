use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Summoner {
    #[serde(rename = "accountId")]
    pub(crate) account_id: Option<i64>,
    #[serde(rename = "displayName")]
    pub(crate) display_name: Option<String>,
    #[serde(rename = "internalName")]
    pub(crate) internal_name: Option<String>,
    #[serde(rename = "nameChangeFlag")]
    pub(crate) name_change_flag: Option<bool>,
    #[serde(rename = "percentCompleteForNextLevel")]
    pub(crate) percent_complete_for_next_level: Option<i64>,
    #[serde(rename = "privacy")]
    pub(crate) privacy: Option<Privacy>,
    #[serde(rename = "profileIconId")]
    pub(crate) profile_icon_id: Option<i64>,
    #[serde(rename = "puuid")]
    pub(crate) puuid: Option<String>,
    #[serde(rename = "rerollPoints")]
    pub(crate) reroll_points: Option<RerollPoints>,
    #[serde(rename = "summonerId")]
    pub(crate) summoner_id: Option<i64>,
    #[serde(rename = "summonerLevel")]
    pub(crate) summoner_level: Option<i64>,
    #[serde(rename = "unnamed")]
    pub(crate) unnamed: Option<bool>,
    #[serde(rename = "xpSinceLastLevel")]
    pub(crate) xp_since_last_level: Option<i64>,
    #[serde(rename = "xpUntilNextLevel")]
    pub(crate) xp_until_next_level: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Privacy {
    PUBLIC,
    PRIVATE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RerollPoints {
    #[serde(rename = "currentPoints")]
    current_points: Option<i32>,
    #[serde(rename = "maxRolls")]
    max_rolls: Option<i32>,
    #[serde(rename = "numberOfRolls")]
    number_of_rolls: Option<i32>,
    #[serde(rename = "pointsCostToRoll")]
    points_cost_to_roll: Option<i32>,
    #[serde(rename = "pointsToReroll")]
    points_to_reroll: Option<i32>,
}
