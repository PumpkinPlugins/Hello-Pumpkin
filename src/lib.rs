use async_trait::async_trait;
use pumpkin::plugin::{player::PlayerJoinEvent, Context, EventHandler, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method, with_runtime};
use pumpkin_util::text::{color::NamedColor, TextComponent};

struct MyJoinHandler;

#[with_runtime(global)]
#[async_trait]
impl EventHandler<PlayerJoinEvent> for MyJoinHandler {
    async fn handle_blocking(&self, event: &mut PlayerJoinEvent) {
        event.join_message =
            TextComponent::text(format!("Welcome, {}!", event.player.gameprofile.name))
                .color_named(NamedColor::Green);
    }
}

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Hello, Pumpkin!");

    server
        .register_event(MyJoinHandler, EventPriority::Lowest, true)
        .await;

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
