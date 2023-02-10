use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Ok(_bot_token) = env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    todo!("Add logic for possible use other client for bot, not only reqwest");
}
