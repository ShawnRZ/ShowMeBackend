// use crate::lcu::parameter::Parameter;
// use crate::lcu::request::get_current_summoner as lcu_get_current_summoner;
// use crate::lcu::request::get_summoner_by_name as lcu_get_summoner_by_name;
// use crate::sgp::request::get_summoner_by_name as sgp_get_summoner_by_name;
use serde::{Deserialize, Serialize};

// type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Summoner {
    #[serde(alias = "accountId")]
    account_id: Option<i64>,

    #[serde(alias = "displayName")]
    #[serde(alias = "name")]
    display_name: Option<String>,

    #[serde(alias = "internalName")]
    internal_name: Option<String>,

    #[serde(alias = "nameChangeFlag")]
    name_change_flag: Option<bool>,

    #[serde(alias = "privacy")]
    privacy: Option<Privacy>,

    #[serde(alias = "profileIconId")]
    profile_icon_id: Option<i64>,

    #[serde(alias = "puuid")]
    puuid: Option<String>,

    #[serde(alias = "summonerId")]
    #[serde(alias = "id")]
    summoner_id: Option<i64>,

    #[serde(alias = "level")]
    #[serde(alias = "summonerLevel")]
    summoner_level: Option<i64>,

    #[serde(alias = "xpSinceLastLevel")]
    xp_since_last_level: Option<i64>,

    #[serde(alias = "xpUntilNextLevel")]
    xp_until_next_level: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "PRIVATE")]
    Private,

    #[serde(rename = "PUBLIC")]
    Public,
}

// impl Summoner {
//     pub async fn my_self() -> Result<Self, Error> {
//         let str = lcu_get_current_summoner().await?;
//         let s: Summoner = serde_json::from_str(&str)?;
//         Ok(s)
//     }

//     pub async fn querry(name: String) -> Result<Self, Error> {
//         let mut j = tokio::task::JoinSet::new();

//         j.spawn(lcu_get_summoner_by_name(name.clone()));

//         let region = Parameter::get().await?.region;
//         j.spawn(sgp_get_summoner_by_name(name, region).into());

//         let s = j.join_next().await.unwrap()??;
//         Ok(s)
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::lcu::parameter::Parameter;
//     use crate::lcu::request::get_current_summoner as lcu_get_current_summoner;
//     use crate::sgp::request::get_summoner_by_name as sgp_get_summoner_by_name;

//     use super::Summoner;

//     #[test]
//     fn lcu_get_current_summoner_t() {
//         tokio_test::block_on(Parameter::refresh()).unwrap();
//         let s = tokio_test::block_on(lcu_get_current_summoner()).unwrap();
//         println!("{:#?}", s);
//     }

//     #[test]
//     fn sgp_get_summoner_by_name_t() {
//         tokio_test::block_on(Parameter::refresh()).unwrap();
//         let res = tokio_test::block_on(sgp_get_summoner_by_name(
//             "fyyyyk".to_string(),
//             "hn1".to_string(),
//         ))
//         .unwrap();
//         let s: Summoner = serde_json::from_str(&res).unwrap();
//         println!("{res}");
//         println!("{:#?}", s);
//     }

//     #[test]
//     fn summoner_query_t() {
//         tokio_test::block_on(Parameter::refresh()).unwrap();
//         let s = tokio_test::block_on(Summoner::querry("fyyyyk".to_string())).unwrap();
//         println!("{:#?}", s);
//     }
// }
