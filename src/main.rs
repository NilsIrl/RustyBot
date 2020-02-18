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
struct ChallengeRequest<'a> {
    r#type: &'a str,
    challenge: &'a str,
}

#[derive(serde::Deserialize)]
struct MemberJoinedChannelRequest<'a> {
    r#type: &'a str,
    event: Event<'a>,
}

#[rocket::post("/", data = "<request>")]
fn member_joined_channel(
    request: rocket_contrib::json::Json<MemberJoinedChannelRequest>,
    sender: rocket::State<crossbeam_channel::Sender<PostMessage>>,
) {
    // TODO: Check that the type is indeed "member_joined_channel" earlier
    assert_eq!(request.r#type, "member_joined_channel");
    sender
        .inner()
        .send(PostMessage {
            channel: request.event.channel.clone(),
            user: request.event.user.clone(),
        })
        .unwrap();
}

#[rocket::post("/", data = "<request>")]
fn challenge(
    request: rocket_contrib::json::Json<ChallengeRequest>,
) -> rocket_contrib::json::Json<ChallengeResponse> {
    rocket_contrib::json::Json(ChallengeResponse {
        challenge: request.challenge.to_string(),
    })
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
        .mount("/", rocket::routes![member_joined_channel, challenge])
        .launch();
}
