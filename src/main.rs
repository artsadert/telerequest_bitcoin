mod modules;


use teloxide::prelude::*;

use modules::{dotenvy::get_token, request::get_price};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("starting bot...");

    let token = get_token();
    let bot = Bot::new(&token);


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_message(msg.chat.id, 
                         format!("It was fucking hard!\nPrice of Bitcoin is {}", get_price().await.unwrap()["price"])).await?;
        println!("{}", msg.chat.id);
        Ok(())
    })
    .await;
}
