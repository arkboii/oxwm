use anyhow::Result;
use std::time::Duration;

mod battery;
mod datetime;
mod ram;
mod shell;

use battery::Battery;
use datetime::DateTime;
use ram::Ram;
use shell::ShellBlock;

pub trait Block {
    fn content(&mut self) -> Result<String>;
    fn interval(&self) -> Duration;
    fn color(&self) -> u32;
}

pub struct BlockConfig {
    pub format: &'static str,
    pub command: BlockCommand,
    pub interval_secs: u64,
    pub color: u32,
    pub underline: bool,
}

pub enum BlockCommand {
    Shell(&'static str),
    DateTime(&'static str),
    Battery {
        format_charging: &'static str,
        format_discharging: &'static str,
        format_full: &'static str,
    },
    Ram,
    Static(&'static str),
}

impl BlockConfig {
    pub fn to_block(&self) -> Box<dyn Block> {
        match self.command {
            BlockCommand::Shell(cmd) => Box::new(ShellBlock::new(
                self.format,
                cmd,
                self.interval_secs,
                self.color,
            )),
            BlockCommand::DateTime(fmt) => Box::new(DateTime::new(
                self.format,
                fmt,
                self.interval_secs,
                self.color,
            )),
            BlockCommand::Battery {
                format_charging,
                format_discharging,
                format_full,
            } => Box::new(Battery::new(
                format_charging,
                format_discharging,
                format_full,
                self.interval_secs,
                self.color,
            )),
            BlockCommand::Ram => Box::new(Ram::new(self.format, self.interval_secs, self.color)),
            BlockCommand::Static(text) => Box::new(StaticBlock::new(
                &format!("{}{}", self.format, text),
                self.color,
            )),
        }
    }
}

struct StaticBlock {
    text: String,
    color: u32,
}

impl StaticBlock {
    fn new(text: &str, color: u32) -> Self {
        Self {
            text: text.to_string(),
            color,
        }
    }
}

impl Block for StaticBlock {
    fn content(&mut self) -> Result<String> {
        Ok(self.text.clone())
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(u64::MAX)
    }

    fn color(&self) -> u32 {
        self.color
    }
}
