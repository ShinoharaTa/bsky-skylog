use aerostream::{api::AppBskyActorDefsProfileview, Client};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    handle: String,
    password: String,
}

async fn get_all_followers(
    client: &Client,
    handle: &str,
) -> Result<Vec<AppBskyActorDefsProfileview>, anyhow::Error> {
    let mut all_followers: Vec<aerostream::api::AppBskyActorDefsProfileview> = Vec::new();
    let mut current_cursor = None;

    loop {
        let response = client
            .client
            .app_bsky_graph_getfollowers(handle, None, current_cursor.as_deref())
            .with_context(|| format!("フォロワー取得できませんでした: {}", handle))?;
        all_followers.extend(response.followers);
        if response.cursor.is_some() {
            current_cursor = response.cursor; // 新しいカーソルで更新
        } else {
            return Ok(all_followers);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 設定取得
    let config_path = "./config.json";
    let file_content = fs::read_to_string(config_path)
        .with_context(|| format!("設定ファイル {} の読み込みに失敗しました", config_path))?;
    let config: Config = serde_json::from_str(&file_content)
        .with_context(|| format!("設定ファイル {} の展開に失敗しました", config_path))?;

    // クライアントの初期設定
    let mut client = Client::default();
    client
        .login(&config.handle, config.password)
        .with_context(|| format!("ログインに失敗しました: {}", config.handle))?;
    let did = match client.get_handle(&config.handle) {
        Ok(r) => r.clone(),
        _ => String::from("UNKNOWN"),
    };

    // 全フォロワー取得
    let followers = get_all_followers(&client, &config.handle).await?;
    println!("{}", followers.len());

    let getRecord = client.client.com_atproto_repo_listrecords(
        &did,
        "app.bsky.feed.post",
        None,
        None,
        None,
        None,
        None,
    );
    //  (&follower.did, None)?;
    println!("{:?}", getRecord);
    for follower in followers {
        // println!("{}");
    }
    Ok(())
}
