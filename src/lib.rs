use dotenv::dotenv;
use github_flows::{
    get_octo,
    octocrab::{models::orgs::Organization, Result as OctoResult},
    GithubLogin::Default,
};
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};
use std::env;

#[no_mangle]
pub fn run() {
    dotenv().ok();

    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    listen_to_channel(&slack_workspace, &slack_channel, |sm| {
        handler(&slack_workspace, &slack_channel, sm)
    });
}
#[tokio::main(flavor = "current_thread")]

async fn handler(workspace: &str, channel: &str, sm: SlackMessage) {
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("bot get".to_string());
    let github_owner = env::var("github_owner").unwrap_or("WasmEdge".to_string());
    let github_repo = env::var("github_repo").unwrap_or("WasmEdge".to_string());

    if sm.text.contains(&trigger_phrase) {
        let octocrab = get_octo(&Default);

        let html_url = forkee.html_url.unwrap().to_string();
        let time = forkee.created_at.expect("time not found");
        let forkee_as_user = forkee.owner.unwrap();

        let org_url = forkee_as_user.organizations_url;
        let forkee_login = forkee_as_user.login;

        let mut email = "".to_string();
        let mut twitter_handle = "".to_string();

        let user_route = format!("users/{forkee_login}");
        let response: OctoResult<User> = octocrab.get(&user_route, None::<&()>).await;
        match response {
            Err(_) => {}
            Ok(user_obj) => {
                email = user_obj.email.unwrap_or("".to_string());
                twitter_handle = user_obj.twitter_username.unwrap_or("".to_string());
            }
        }

        let mut org_name = "".to_string();
        let mut org_company = "".to_string();

        let org_route = format!("orgs/{forkee_login}");
        let response: OctoResult<Organization> = octocrab.get(&org_route, None::<&()>).await;
        match response {
            Err(_) => {}
            Ok(org_obj) => {
                org_name = org_obj.name.unwrap_or("no org name".to_string());
                org_company = org_obj.company.unwrap_or("no company name".to_string());
            }
        };

        let data = serde_json::json!({
        "Name": forkee_login,
        "Repo": html_url,
        "Email": email,
        "Twitter": twitter_handle,
        "OrgName": org_name,
        "OrgCompany": org_company,
        "Org": org_url,
        "Created": time,
        });
        send_message_to_channel(&workspace, &channel, data.to_string());
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,
    id: u32,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    organizations_url: String,
    blog: String,
    twitter_username: Option<String>,
    email: Option<String>,
    followers: u32,
}
