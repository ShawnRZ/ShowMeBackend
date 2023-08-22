use super::{
    match_history::{
        LolMatchHistoryMatchHistoryGame, LolMatchHistoryMatchHistoryGameList, MatchHistory,
    },
    parameter::Parameter,
    ranked_stats::RankedStats,
    summoner::Summoner,
};
use reqwest::{Client, RequestBuilder};
use serde_json::Value;

type Error = Box<dyn std::error::Error + Send + Sync>;

async fn get(path: &str) -> Result<RequestBuilder, Error> {
    let lp = Parameter::get().await?;
    let token = lp.token;
    let port = lp.port;
    let req = Client::new()
        .get(std::format!("https://127.0.0.1:{}{}", port, path))
        .basic_auth("riot", Some(token));
    Ok(req)
}

pub async fn get_current_summoner() -> Result<Summoner, Error> {
    let res = get("/lol-summoner/v1/current-summoner")
        .await?
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let s: Summoner = serde_json::from_str(&res)?;
    Ok(s)
    // todo!()
}

pub async fn get_summoner_by_name(name: String) -> Result<Summoner, Error> {
    let res = get("/lol-summoner/v1/summoners")
        .await?
        .query(&[("name", name)])
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let s: Summoner = serde_json::from_str(&res)?;
    Ok(s)
    // todo!()
}

pub async fn get_my_match_history(beg: u64, end: u64) -> Result<MatchHistory, Error> {
    let res = get("/lol-match-history/v1/products/lol/current-summoner/matches")
        .await?
        .query(&[("begIndex", beg), ("endIndex", end)])
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let mh: MatchHistory = serde_json::from_str(&res)?;
    Ok(mh)
    // todo!()
}

pub async fn get_match_history(puuid: String, beg: u64, end: u64) -> Result<MatchHistory, Error> {
    let path = std::format!("/lol-match-history/v1/products/lol/{puuid}/matches");
    let res = get(&path)
        .await?
        .query(&[("begIndex", beg), ("endIndex", end)])
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let mh: MatchHistory = serde_json::from_str(&res)?;
    Ok(mh)
    // todo!()
}

pub async fn get_match_by_id(match_id: u64) -> Result<MatchHistory, Error> {
    let path = std::format!("/lol-match-history/v1/games/{match_id}");
    let res = get(&path).await?.send().await?.error_for_status()?;
    let res = res.text().await?;
    let m: LolMatchHistoryMatchHistoryGame = serde_json::from_str(&res)?;
    let m = MatchHistory {
        account_id: None,
        games: Some(LolMatchHistoryMatchHistoryGameList {
            game_begin_date: None,
            game_count: Some(1),
            game_end_date: None,
            game_index_begin: None,
            game_index_end: None,
            games: Some(vec![m]),
        }),
        platform_id: None,
    };
    Ok(m)
}

pub async fn get_my_ranked_stats() -> Result<RankedStats, Error> {
    let res = get("/lol-ranked/v1/current-ranked-stats")
        .await?
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let r: RankedStats = serde_json::from_str(&res)?;
    Ok(r)
    // todo!()
}

pub async fn get_ranked_stats(puuid: &str) -> Result<RankedStats, Error> {
    let path = std::format!("/lol-ranked/v1/ranked-stats/{puuid}");
    let res = get(&path).await?.send().await?.error_for_status()?;
    let res = res.text().await?;
    let r: RankedStats = serde_json::from_str(&res)?;
    Ok(r)
    // todo!()
}

pub async fn get_sgp_token() -> Result<String, Error> {
    let res = get("/entitlements/v1/token")
        .await?
        .send()
        .await?
        .error_for_status()?;
    let res: Value = serde_json::from_str(&res.text().await?)?;
    let token = res
        .get("accessToken")
        .ok_or("请求成功，但是返回值中却不包含有效信息。请联系作者。".to_string())?
        .as_str()
        .unwrap();

    Ok(token.to_string())
    // todo!()
}

pub async fn get_champ_select_session() -> Result<String, Error> {
    let res = get("/lol-champ-select-legacy/v1/session")
        .await?
        .send()
        .await?
        .error_for_status()?;
    let res = res.text().await?;
    let res: Value = serde_json::from_str(&res)?;

    let mut my_team = Vec::new();
    for v in res["myTeam"]
        .as_array()
        .ok_or("myTeam 字段不存在或不是数组")?
    {
        let cell_id = v["cellId"].as_i64();
        let champion_id = v["championId"].as_i64();
        let team = v["team"].as_i64();
        let summoner_id = v["summonerId"].as_i64();

        let player = serde_json::json!({
            "cellId": cell_id,
            "championId": champion_id,
            "team": team,
            "summonerId": summoner_id,
        });
        my_team.push(player);
    }

    let mut their_team = Vec::new();
    for v in res["myTeam"]
        .as_array()
        .ok_or("theirTeam 字段不存在或不是数组")?
    {
        let cell_id = v["cellId"].as_i64();
        let champion_id = v["championId"].as_i64();
        let team = v["team"].as_i64();
        let summoner_id = v["summonerId"].as_i64();

        let player = serde_json::json!({
            "cellId": cell_id,
            "championId": champion_id,
            "team": team,
            "summonerId": summoner_id,
        });
        their_team.push(player);
    }

    let res = serde_json::json!({
        "myTeam": my_team,
        "theirTeam": their_team,
    });

    Ok(res.to_string())
    // todo!()
}

pub async fn get_summoner_by_id(id: u64) -> Result<Summoner, Error> {
    let path = std::format!("/lol-summoner/v1/summoners/{id}");
    let res = get(&path).await?.send().await?.error_for_status()?;
    let res = res.text().await?;
    let s: Summoner = serde_json::from_str(&res)?;
    Ok(s)
}

pub async fn spectate(puuid: &str) -> Result<(), Error> {
    let lp = Parameter::get().await?;
    let token = lp.token;
    let port = lp.port;
    Client::new()
        .post(std::format!(
            "https://127.0.0.1:{}{}",
            port,
            "/lol-gameflow/v2/spectate/launch"
        ))
        .basic_auth("riot", Some(token))
        .body(std::format!(
            r#"{{
            "gameQueueType": "",
            "dropInSpectateGameId": "",
            "puuid": "{puuid}",
            "allowObserveMode": ""
        }}"#
        ))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_match_by_id() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let s = tokio_test::block_on(get_match_by_id(8201764023)).unwrap();
        println!("{:#?}", s);
    }

    #[test]
    fn test_get_summoner_by_name() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let s = tokio_test::block_on(get_summoner_by_name("fyyyyk".to_string())).unwrap();
        println!("{:#?}", s);
    }

    #[test]
    fn test_get_current_summoner() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let s = tokio_test::block_on(get_current_summoner()).unwrap();
        println!("{:#?}", s);
    }

    #[test]
    fn test_get_my_match_history() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_my_match_history(0, 1)).unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_get_match_history() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_match_history(
            "ae3ac102-09f4-54c7-81e4-384947927e35".to_string(),
            0,
            1,
        ))
        .unwrap();
        println!("{:#?}", res);
    }

    #[test]
    fn test_get_my_ranked_stats() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let r = tokio_test::block_on(get_my_ranked_stats()).unwrap();
        println!("{:#?}", r);
    }

    #[test]
    fn test_get_ranked_stats() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let r =
            tokio_test::block_on(get_ranked_stats("55e4504d-b52e-526f-ac89-ee1eeebd842b")).unwrap();
        println!("{:#?}", r);
    }

    #[test]
    fn test_get_sgp_token() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_sgp_token()).unwrap();
        println!("{}", res);
    }

    #[test]
    fn test_get_champ_select_session() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let res = tokio_test::block_on(get_champ_select_session()).unwrap();
        println!("{}", res);
    }

    #[test]
    fn test_get_summoner_by_id() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let s = tokio_test::block_on(get_summoner_by_id(4011172159)).unwrap();
        println!("{:#?}", s);
    }

    #[test]
    fn test_spectate() {
        tokio_test::block_on(Parameter::refresh()).unwrap();
        let s = tokio_test::block_on(spectate("ae3ac102-09f4-54c7-81e4-384947927e35")).unwrap();
        println!("{:#?}", s);
    }
}
