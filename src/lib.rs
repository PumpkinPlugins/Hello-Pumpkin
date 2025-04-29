use std::sync::Arc;

use async_trait::async_trait;
use pumpkin::{
    command::{
        args::ConsumedArgs, dispatcher::CommandError, tree::builder::literal, tree::CommandTree,
        CommandExecutor, CommandSender,
    },
    plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority},
    server::Server,
};
use pumpkin_api_macros::{plugin_impl, plugin_method, with_runtime};
use pumpkin_util::text::{color::NamedColor, TextComponent};
use pumpkin_util::PermissionLvl;
use rand::{rng, Rng};

struct MyJoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    async fn handle_blocking(&self, _server: &Arc<Server>, event: &mut PlayerJoinEvent) {
        event.join_message =
            TextComponent::text(format!("Welcome, {}!", event.player.gameprofile.name))
                .color_named(NamedColor::Green);
    }
}

const NAMES: [&str; 2] = ["rps", "rockpaperscissors"];
const DESCRIPTION: &str = "Play Rock Paper Scissors with the server.";

struct RockPaperScissorsExecutor(Choice);

#[async_trait]
impl CommandExecutor for RockPaperScissorsExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player_choice = self.0;
        let computer_choice = get_random_choice();

        sender
            .send_message(
                TextComponent::text("You chose: ")
                    .add_text(format!("{:?}", player_choice))
                    .color_named(NamedColor::Aqua),
            )
            .await;

        sender
            .send_message(
                TextComponent::text("I chose: ")
                    .add_text(format!("{:?}", computer_choice))
                    .color_named(NamedColor::Gold),
            )
            .await;

        match player_choice.beats(&computer_choice) {
            Outcome::Win => {
                sender
                    .send_message(TextComponent::text("You win!").color_named(NamedColor::Green))
                    .await;
            }
            Outcome::Lose => {
                sender
                    .send_message(TextComponent::text("You lose!").color_named(NamedColor::Red))
                    .await;
            }
            Outcome::Draw => {
                sender
                    .send_message(
                        TextComponent::text("It's a tie!").color_named(NamedColor::Yellow),
                    )
                    .await;
            }
        }

        Ok(())
    }
}

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    log::info!("Hello, Pumpkin!");

    server
        .register_event(Arc::new(MyJoinHandler), EventPriority::Lowest, true)
        .await;

    let command = CommandTree::new(NAMES, DESCRIPTION)
        .then(literal("rock").execute(RockPaperScissorsExecutor(Choice::Rock)))
        .then(literal("paper").execute(RockPaperScissorsExecutor(Choice::Paper)))
        .then(literal("scissors").execute(RockPaperScissorsExecutor(Choice::Scissors)));

    server.register_command(command, PermissionLvl::Zero).await;

    Ok(())
}

#[plugin_impl]
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Choice {
    pub fn beats(&self, other: &Choice) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

fn get_random_choice() -> Choice {
    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    let index = rng().random_range(0..3);
    choices[index]
}
