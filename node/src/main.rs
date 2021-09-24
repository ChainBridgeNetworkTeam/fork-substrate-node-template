//! Substrate Node Template CLI library.
//! 开启告警，没有文档warning
#![warn(missing_docs)]

mod chain_spec;

//	导出宏
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
