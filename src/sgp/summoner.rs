use crate::lcu::summoner::Privacy as LcuPrivacy;
use crate::lcu::summoner::Summoner as LcuSummoner;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Summoner {
    #[serde(rename = "id")]
    id: Option<i64>,
    #[serde(rename = "puuid")]
    puuid: Option<String>,
    #[serde(rename = "accountId")]
    account_id: Option<i64>,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "internalName")]
    internal_name: Option<String>,
    #[serde(rename = "profileIconId")]
    profile_icon_id: Option<i64>,
    #[serde(rename = "level")]
    level: Option<i64>,
    #[serde(rename = "expPoints")]
    exp_points: Option<i64>,
    #[serde(rename = "levelAndXpVersion")]
    level_and_xp_version: Option<i64>,
    #[serde(rename = "revisionId")]
    revision_id: Option<i64>,
    #[serde(rename = "revisionDate")]
    revision_date: Option<i64>,
    #[serde(rename = "accolastGameDateuntId")]
    last_game_date: Option<i64>,
    #[serde(rename = "nameChangeFlag")]
    name_change_flag: Option<bool>,
    #[serde(rename = "unnamed")]
    unnamed: Option<bool>,
    #[serde(rename = "privacy")]
    privacy: Option<Privacy>,
    #[serde(rename = "expToNextLevel")]
    exp_to_next_level: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Privacy {
    PUBLIC,
    PRIVATE,
}

impl Into<LcuPrivacy> for Privacy {
    fn into(self) -> LcuPrivacy {
        match self {
            Privacy::PRIVATE => LcuPrivacy::PRIVATE,
            Privacy::PUBLIC => LcuPrivacy::PUBLIC,
        }
    }
}

impl Into<LcuSummoner> for Summoner {
    fn into(self) -> LcuSummoner {
        LcuSummoner {
            account_id: self.account_id,
            display_name: self.name,
            internal_name: self.internal_name,
            name_change_flag: self.name_change_flag,
            percent_complete_for_next_level: if self.exp_points.is_some()
                && self.exp_to_next_level.is_some()
            {
                Some(self.exp_points.unwrap() / self.exp_to_next_level.unwrap())
            } else {
                None
            },
            privacy: if self.privacy.is_some() {
                Some(self.privacy.unwrap().into())
            } else {
                None
            },
            profile_icon_id: self.profile_icon_id,
            puuid: self.puuid,
            reroll_points: None,
            summoner_id: self.id,
            summoner_level: self.level,
            unnamed: self.unnamed,
            xp_since_last_level: self.exp_points,
            xp_until_next_level: self.exp_to_next_level,
        }
    }
}
