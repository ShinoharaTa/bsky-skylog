use aerostream::{
    api::{AppBskyActorDefsProfileview, AppBskyGraphGetfollowers},
    Client,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Config {
    handle: String,
    password: String,
}

async fn get_all_followers(client: &Client, handle: &str) -> Vec<AppBskyActorDefsProfileview> {
    let mut all_followers: Vec<aerostream::api::AppBskyActorDefsProfileview> = Vec::new();
    let mut current_cursor = None;

    loop {
        match client
            .client
            .app_bsky_graph_getfollowers(handle, None, current_cursor.as_deref())
        {
            Ok(response) => {
                all_followers.extend(response.followers);
                if response.cursor.is_some() {
                    current_cursor = response.cursor; // 新しいカーソルで更新
                } else {
                    break; // カーソルがNoneなら全フォロワーの取得完了
                }
            }
            _ => break, // エラーを返す
        }
    }

    return all_followers;
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config_path = "./config.json";
    let file_content = fs::read_to_string(config_path)
        .with_context(|| format!("設定ファイル {} の読み込みに失敗しました", config_path))?;
    let config: Config = serde_json::from_str(&file_content)
        .with_context(|| format!("設定ファイル {} の展開に失敗しました", config_path))?;

    let mut client = Client::default();
    client.set_timeout(5);
    client.login(&config.handle, config.password);
    let did = match client.get_handle(&config.handle) {
        Ok(r) => r.clone(),
        _ => String::from("UNKNOWN"),
    };
    let followers: Vec<AppBskyActorDefsProfileview> =
        get_all_followers(&client, &config.handle).await;
    println!("{}", followers.len());
    Ok(())
    // let handle = client.get_repo(did);
}
