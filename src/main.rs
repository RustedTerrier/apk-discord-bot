use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        macros::{command, group, help, check, hook},
    },
    http::Http,
};
use unicode_segmentation::UnicodeSegmentation; // 1.6.0
use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};

const PREFIX: &str = "apk ";

pub struct MessageId(pub u64);

#[group]
#[only_in(guild)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //If the first 4 letters of the message are the prefix
        if msg.content.chars().count() > 4 {
            if &msg.content[0..4] == PREFIX {
                match &msg.content[4..] {//Cycle through the commands and see if it is a command
                    "info" => {
                        if let Err(why) = msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m.embed(|e| {
                                    e.title("**Apk help**");
                                    e.description(
                                        "Apk is a discord bot to moderate *The Opposition*. The prefix is `apk`.

                                        **Commands**
                                        `apk ping`
                                        `apk mute [username]`
                                        `apk kick [username]`
                                        `apk ban [username]`
                                        `apk purge <number of lines>`",
                                    );

                                    e
                                });

                                m
                            })
                            .await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    "memes" => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "memes").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    "ping" => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    _      => {if &msg.content[4..9] == "purge " {}
                                
                    }
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    let token = env::var("DISCORD_TOKEN").expect(
        "Expected a token in the environment",
    );

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    
    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix("apk ")
                   // In this case, if "," would be first, a message would never
                   // be delimited at ", ", forcing you to trim your arguments if you
                   // want to avoid whitespaces at the start of each.
                   .delimiters(vec![", ", ","])
                   // Sets the bot's owners. These will be used for commands that
                   // are owners only.
                   .owners(owners));
    let mut client = Client::builder(&token).event_handler(Handler).framework(framework).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
