#![feature(proc_macro_hygiene, decl_macro)]

#[derive(serde::Serialize)]
struct ChallengeResponse {
    challenge: String,
}

#[derive(serde::Serialize)]
struct PostMessageRequest<'a> {
    channel: &'a str,
    text: String,
    as_user: bool,
}

#[derive(serde::Deserialize)]
struct Event<'a> {
    user: String,
    channel: String,
    r#type: &'a str,
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum EventRequest<'a> {
    ChallengeRequest { r#type: String, challenge: String },
    MemberJoinedChannelRequest { r#type: &'a str, event: Event<'a> },
}

#[derive(rocket::response::Responder)]
enum EventResponse {
    Challenge(rocket_contrib::json::Json<ChallengeResponse>),
    Status(rocket::http::Status),
}

#[rocket::post("/", data = "<request>")]
fn event(
    request: rocket_contrib::json::Json<EventRequest>,
    token: rocket::State<String>,
) -> EventResponse {
    match &*request {
        EventRequest::ChallengeRequest { challenge, .. } => {
            EventResponse::Challenge(rocket_contrib::json::Json(ChallengeResponse {
                challenge: challenge.to_string(),
            }))
        }
        EventRequest::MemberJoinedChannelRequest {
            // Is this really the best event to be subscribed to? `team_join` might be interesting as well.
            event:
                Event {
                    user,
                    channel,
                    r#type: "member_joined_channel",
                },
            ..
        } => {
            welcome_user(user, channel, &*token);
            EventResponse::Status(rocket::http::Status::Ok)
        }
        EventRequest::MemberJoinedChannelRequest { .. } => unreachable!(),
    }
}

fn welcome_user(user: &str, channel: &str, token: &String) {
    let client = reqwest::blocking::Client::new(); // TODO: create client elsewhere
    client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .json(&PostMessageRequest {
            channel: &channel,
            text: format!("Hello <@{}>, welcome!!! :wave:", user),
            as_user: true,
        })
        .send()
        .unwrap();
}

fn main() {
    let token = std::env::var("SLACK_TOKEN").unwrap();
    rocket::ignite()
        .manage(token)
        .mount("/", rocket::routes![event])
        .launch();
}
