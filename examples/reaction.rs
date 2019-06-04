extern crate slack;
use slack::{
    Event::{self, ReactionAdded},
    RtmClient,
};

struct MyHandler;

impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, _cli: &RtmClient, event: Event) {
        match event {
            ReactionAdded { .. } => {
                println!("got reactionadded: {:?}", event);
            }
            _ => (),
        };
    }

    fn on_close(&mut self, _cli: &RtmClient) {}

    fn on_connect(&mut self, _cli: &RtmClient) {}
}

fn main() {
    let api_key_name = "SLACK_API_KEY";
    let api_key = std::env::vars()
        .find(|(k, _)| k == api_key_name)
        .map(|(_, v)| v);

    let api_key = match api_key {
        Some(k) => k,
        None => {
            eprintln!("no {} found", api_key_name);
            std::process::exit(2);
        }
    };

    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
