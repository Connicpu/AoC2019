pub use self::cpu::{Cpu, CpuResult};
pub use self::io::{ChannelIO, SingleIO, StdIO, IO};
pub use crate::parse::parse_i64_vec as parse;

pub mod cpu;
pub mod io;
#[cfg(test)]
mod tests;
