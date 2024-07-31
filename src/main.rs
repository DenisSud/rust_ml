// src/main.rs

mod data;
mod analysis;
mod ui;

use iced::{Settings, Application};
use ui::SmallBizAnalytics;

fn main() -> iced::Result {
    SmallBizAnalytics::run(Settings::default())
}
