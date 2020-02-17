#![feature(proc_macro_hygiene, decl_macro)]

lazy_static::lazy_static! {
    static ref TOKEN: String = std::env::var("SLACK_TOKEN").unwrap(); // Would use state rather than lazy_static be better?
}

#[derive(serde::Serialize)]
struct ChallengeResponse {
    challenge: String,
}

#[derive(serde::Serialize)]
struct PostMessageRequest<'a> {
    channel: &'a str,
    text: String,
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
fn event(request: rocket_contrib::json::Json<EventRequest>) -> EventResponse {
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
            welcome_user(user, channel);
            EventResponse::Status(rocket::http::Status::Ok)
        }
        EventRequest::MemberJoinedChannelRequest { .. } => unreachable!(),
    }
}

fn welcome_user(user: &str, channel: &str) {
    let client = reqwest::blocking::Client::new(); // TODO: create client elsewhere
    client
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(&*TOKEN)
        .json(&PostMessageRequest {
            channel: &channel,
            text: format!("Hello <@{}>, welcome!!!", user),
        })
        .send()
        .unwrap();
}

fn main() {
    rocket::ignite().mount("/", rocket::routes![event]).launch();
}
