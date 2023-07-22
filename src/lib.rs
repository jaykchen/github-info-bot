use dotenv::dotenv;
use github_flows::{
    get_octo,
    octocrab::{models::orgs::Organization, Result as OctoResult},
    GithubLogin,
};
use openai_flows::{chat, chat::ChatModel, chat::ChatOptions, OpenAIFlows};
use serde::{Deserialize, Serialize};
use serde_json;
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    let slack_workspace = env::var("slack_workspace").unwrap_or("secondstate".to_string());
    let slack_channel = env::var("slack_channel").unwrap_or("github-status".to_string());

    listen_to_channel(&slack_workspace, &slack_channel, |sm| {
        handler(&slack_workspace, &slack_channel, sm)
    })
    .await;
}

async fn handler(workspace: &str, channel: &str, sm: SlackMessage) {
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("bot@get".to_string());
    // let github_owner = env::var("github_owner").unwrap_or("WasmEdge".to_string());
    // let github_repo = env::var("github_repo").unwrap_or("WasmEdge".to_string());

    let parts: Vec<&str> = sm
        .text
        .split("bot@get")
        .nth(1) // skip the part before "bot@get"
        .unwrap_or("") // if "bot@get" is not found, use an empty string
        .split_whitespace()
        .collect();

    let (github_owner, github_repo, user_name) = match parts.as_slice() {
        [owner, repo, user, ..] => (owner, repo, user),
        _ => panic!("Input should contain 'bot@get <github_owner> <github_repo> <user_name>'"),
    };

    let mut out = String::from("placeholder");
    if sm.text.contains(&trigger_phrase) {
        let openai = OpenAIFlows::new();

        let sys_prompt_1 = &format!("Given the information that user '2019zhou' opened an issue titled 'asking for guidance on how to open the debug log for a specific function', your task is to analyze the content of the issue posts. Extract key details including the main problem or question raised, the environment in which the issue occurred, any steps taken by the user to address the problem, relevant discussions, and any identified solutions or pending tasks.");

        let chat_id = format!("issue_2019");

        let co_1 = ChatOptions {
            model: ChatModel::GPT35Turbo16K,
            restart: false,
            system_prompt: Some(sys_prompt_1),
            max_tokens: Some(256),
            ..Default::default()
        };

        let usr_prompt_1 = &format!("Based on the GitHub issue posts: 2019zhou commented on Nov 7, 2022
How to open the debug log of the function?
@alabulei1
Contributor
alabulei1 commented on Nov 8, 2022
@hydai Please help take a look. @2019zhou wants to open the debug log for HTTPS plugin
@hydai
Member
hydai commented on Nov 8, 2022
What kind of debug log do you need?
If you are using libwasmedge, please refer to this function to enable the debug log level: https://github.com/WasmEdge/WasmEdge/blob/master/include/api/wasmedge/wasmedge.h#L240
Otherwise, you have to modify the wasmedge toolchain and call this function instead: https://github.com/WasmEdge/WasmEdge/blob/master/include/common/log.h#L25, please list the following key details: The main problem or question raised in the issue. The environment or conditions in which the issue occurred (e.g., hardware, OS). Any steps or actions taken by the user '2019zhou' or others to address the issue. Key discussions or points of view shared by participants in the issue thread. Any solutions identified, or pending tasks if the issue hasn't been resolved. The role and contribution of the user '2019zhou' in the issue.");

        let system_obj = serde_json::json!(
            {"role": "system", "content": sys_prompt_1}
        );

        let user_obj_1 = serde_json::json!(
            {"role": "user", "content": usr_prompt_1}
        );

        if let Ok(res) = openai.chat_completion(&chat_id, usr_prompt_1, &co_1).await {
            let assistant_obj = serde_json::json!(
                {"role": "assistant", "content": &res.choice}
            );
            let usr_prompt_2 = &format!("Based on the key details listed in the previous step, provide a high-level summary of the issue <Brief summary of the main problem, steps taken, discussions, and current status of the issue>. Highlight the role and contribution of '2019zhou'");

            // let user_obj_2 = serde_json::json!(
            //     {"role": "user", "content": usr_prompt_2}
            // );
            let sys_prompt_2 = serde_json::json!([system_obj, user_obj_1, assistant_obj]);
            let temp = sys_prompt_2.to_string();
            let co_2 = ChatOptions {
                model: ChatModel::GPT35Turbo16K,
                restart: false,
                system_prompt: Some(&temp),
                max_tokens: Some(128),
                ..Default::default()
            };
            if let Ok(res) = openai.chat_completion(&chat_id, usr_prompt_2, &co_2).await {
                out.push(' ');
                // out.push_str(&html_url);
                out.push(' ');
                out.push_str(&res.choice);
                println!("{:?}", out);
                send_message_to_channel("ik8", "ch_out", out).await;
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct UserProfile {
    login: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    organizations_url: String,
    blog: String,
    twitter_username: Option<String>,
    email: Option<String>,
    followers: u32,
    stargazers_count: u32,
    rank_status: String,
    influence_status: String,
}

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

pub fn is_top_by_contribution() {

    // https://github.com/search?q=followers%3A%3E1000&type=Users
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarGazer {
    login: String,
    id: u64,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    starred_url: String,
    organizations_url: String,
    repos_url: String,
}
