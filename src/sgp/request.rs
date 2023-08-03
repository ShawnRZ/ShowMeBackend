use super::{
    match_history::{Game, MatchHistory},
    summoner::Summoner,
};
use crate::lcu::request::get_sgp_token;
use crate::lcu::summoner::Summoner as LcuSummoner;
use crate::lcu::{self, match_history::MatchHistory as LcuMatchHistory};
use reqwest::{Client, IntoUrl, RequestBuilder};
use serde_json::Value;
use std::{collections::HashMap, mem::MaybeUninit, sync::Once};

type Error = Box<dyn std::error::Error + Send + Sync>;

async fn get<U: IntoUrl>(url: U) -> Result<RequestBuilder, Error> {
    let token = get_sgp_token().await?;
    Ok(Client::new().get(url).bearer_auth(token))
}

#[allow(dead_code)]
async fn get_region_path() -> Result<Vec<Value>, Error> {
    let res = Client::new()
        .get("https://prod-rso.lol.qq.com:3000/.well-known/openid-configuration")
        .send()
        .await?;
    let res: Value = serde_json::from_str(&res.text().await?)?;
    let res = res["riot_lol_regions_supported"]
        .as_array()
        .unwrap()
        .to_owned();
    Ok(res)
    // todo!()
}

fn get_server_info(region: &str) -> Result<(&'static str, &'static str), Error> {
    let region = region.to_string().to_lowercase();
    let sa = get_servers()?;
    let a = sa
        .get(region.as_str())
        .ok_or(std::format!("服务器{region}不存在"))?;
    Ok(*a)
}

fn get_servers() -> Result<&'static HashMap<&'static str, (&'static str, &'static str)>, Error> {
    static mut SERVER_ADDRESS: MaybeUninit<HashMap<&str, (&str, &str)>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        SERVER_ADDRESS.as_mut_ptr().write(HashMap::from([
            ("hn1", ("https://hn1-cloud-sgp.lol.qq.com:21019", "HN1")),
            ("hn2", ("https://hn2-sgp.lol.qq.com:21019", "HN2")),
            ("hn3", ("https://hn3-cloud-sgp.lol.qq.com:21019", "HN3")),
            ("hn4", ("https://hn4new-sgp.lol.qq.com:21019", "HN4_NEW")),
            ("hn5", ("https://hn5-sgp.lol.qq.com:21019", "HN5")),
            ("hn6", ("https://hn6-sgp.lol.qq.com:21019", "HN6")),
            ("hn7", ("https://hn7-sgp.lol.qq.com:21019", "HN7")),
            ("hn8", ("https://hn8-cloud-sgp.lol.qq.com:21019", "HN8")),
            ("hn9", ("https://hn9-sgp.lol.qq.com:21019", "HN9")),
            ("hn10", ("https://hn10-cloud-sgp.lol.qq.com:21019", "HN10")),
            ("hn11", ("https://hn11-cloud-sgp.lol.qq.com:21019", "HN11")),
            ("hn12", ("https://hn12-sgp.lol.qq.com:21019", "HN12")),
            ("hn13", ("https://hn13-sgp.lol.qq.com:21019", "HN13")),
            ("hn14", ("https://hn14new-sgp.lol.qq.com:21019", "HN14_NEW")),
            ("hn15", ("https://hn15-sgp.lol.qq.com:21019", "HN15")),
            ("hn16", ("https://hn16new-sgp.lol.qq.com:21019", "HN16_NEW")),
            ("hn17", ("https://hn17-cloud-sgp.lol.qq.com:21019", "HN17")),
            (
                "hn18",
                ("https://hn18-cloud-sgp.lol.qq.com:21019", "HN18_NEW"),
            ),
            ("hn19", ("https://hn19-sgp.lol.qq.com:21019", "HN19")),
            ("wt1", ("https://wt1new-sgp.lol.qq.com:21019", "WT1_NEW")),
            ("wt2", ("https://wt2new-sgp.lol.qq.com:21019", "WT2_NEW")),
            ("wt3", ("https://wt3new-sgp.lol.qq.com:21019", "WT3_NEW")),
            ("wt4", ("https://wt4new-sgp.lol.qq.com:21019", "WT4_NEW")),
            ("wt5", ("https://wt5-sgp.lol.qq.com:21019", "WT5")),
            ("wt6", ("https://wt6-sgp.lol.qq.com:21019", "WT6")),
            ("wt7", ("https://wt7-sgp.lol.qq.com:21019", "WT7")),
            ("sgp1", ("https://bgp1-sgp.lol.qq.com:21019", "BGP1")),
            ("sgp2", ("https://bgp2-sgp.lol.qq.com:21019", "BGP2")),
            ("edu1", ("https://edu1-sgp.lol.qq.com:21019", "EDU1")),
        ]))
    });
    unsafe { Ok(&*SERVER_ADDRESS.as_ptr()) }
}

pub async fn get_match_history(
    puuid: String,
    start: u64,
    count: u64,
) -> Result<LcuMatchHistory, Error> {
    let path = std::format!("/match-history-query/v1/products/lol/player/{puuid}/SUMMARY");
    let region = lcu::parameter::Parameter::get().await?.region;
    let host = get_server_info(&region)?.0;
    let url = std::format!("{host}{path}");
    let res = get(url)
        .await?
        .query(&[("startIndex", start), ("count", count)])
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    // println!("{}", &res);
    let mut mh: MatchHistory = serde_json::from_str(&res)?;
    // 删除其他玩家信息
    for g in mh.games.as_mut().unwrap().iter_mut() {
        for (i, p) in g
            .metadata
            .as_ref()
            .unwrap()
            .participants
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .rev()
        {
            if *p == puuid {
                let s = g
                    .json
                    .as_mut()
                    .unwrap()
                    .participants
                    .as_mut()
                    .unwrap()
                    .remove(i);
                g.json.as_mut().unwrap().participants = Some(vec![s]);
                break;
            }
        }
    }
    let mh: LcuMatchHistory = mh.into();
    Ok(mh)
}

pub async fn get_summoner_by_name(name: String, region: String) -> Result<LcuSummoner, Error> {
    let server = get_server_info(&region)?;
    let host = server.0;
    let path = std::format!(
        "/summoner-ledge/v1/regions/{}/summoners/name/{}",
        server.1,
        name
    );
    let url = std::format!("{host}{path}");
    let res = get(url).await?.send().await?.error_for_status()?;
    let res = res.text().await?;
    let s: Summoner = serde_json::from_str(&res)?;
    Ok(s.into())
}

pub async fn get_ranked_stats(puuid: &str, region: &str) -> Result<String, Error> {
    let server = get_server_info(region)?;
    let host = server.0;
    let url = std::format!("{host}/leagues-ledge/v2/rankedStats/puuid/{puuid}");
    let res = get(url).await?.send().await?;
    Ok(res.text().await?)
}

pub async fn get_match_by_id(match_id: u64) -> Result<LcuMatchHistory, Error> {
    let region = &lcu::parameter::Parameter::get().await?.region;
    let server = get_server_info(region)?;
    let host = server.0;
    let match_id = std::format!("{}_{}", server.1, match_id);
    let url = std::format!("{host}/match-history-query/v1/products/lol/{match_id}/SUMMARY");
    let res = get(url).await?.send().await?.text().await?;
    let g: Game = serde_json::from_str(&res)?;
    let mh = MatchHistory {
        games: Some(vec![g]),
    };
    let mh: LcuMatchHistory = mh.into();
    Ok(mh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lcu::parameter::Parameter;

    #[test]
    fn test_get_match_by_id() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_match_by_id(8201764023)).unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_get_server_address() {
        let res = get_server_info("edu1").unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_get_match_history_by_puuid() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let mh = tokio_test::block_on(get_match_history(
            "55e4504d-b52e-526f-ac89-ee1eeebd842b".to_owned(),
            0,
            1,
        ))
        .unwrap();
        println!("{:#?}", mh);
    }

    #[test]
    fn test_get_region_path() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_region_path()).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_get_summoner_by_name() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_summoner_by_name(
            "丨AvadaKedavra丨".to_owned(),
            "hn1".to_string(),
        ))
        .unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_get_ranked_stats() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_ranked_stats(
            "55e4504d-b52e-526f-ac89-ee1eeebd842b",
            "hn1",
        ))
        .unwrap();
        println!("{:?}", res);
    }
}
