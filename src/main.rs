#![feature(proc_macro_hygiene, decl_macro)]

#[derive(serde::Serialize)]
struct ChallengeResponse {
    challenge: String,
}

struct PostMessage {
    channel: String,
    user: String,
}

#[derive(serde::Serialize)]
struct PostMessageRequest<'a> {
    channel: &'a str,
    text: &'a str,
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
fn event<'a>(
    request: rocket_contrib::json::Json<EventRequest<'a>>,
    sender: rocket::State<crossbeam_channel::Sender<PostMessage>>,
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
            sender
                .inner()
                .send(PostMessage {
                    channel: channel.clone(),
                    user: user.clone(),
                })
                .unwrap();
            EventResponse::Status(rocket::http::Status::Ok)
        }
        EventRequest::MemberJoinedChannelRequest { .. } => unreachable!(),
    }
}

fn main() {
    let (s, r): (
        crossbeam_channel::Sender<PostMessage>,
        crossbeam_channel::Receiver<PostMessage>,
    ) = crossbeam_channel::unbounded();
    let client = reqwest::blocking::Client::new();
    let token = std::env::var("SLACK_TOKEN").unwrap();
    std::thread::spawn(move || loop {
        let post_message = r.recv().unwrap();
        client
            .post("https://slack.com/api/chat.postMessage")
            .bearer_auth(&token)
            .json(&PostMessageRequest {
                channel: &post_message.channel,
                text: &format!("Hello <@{}>, welcome!!! :wave:", post_message.user),
                as_user: true,
            })
            .send()
            .unwrap();
    });
    rocket::ignite()
        .manage(s)
        .mount("/", rocket::routes![event])
        .launch();
}
