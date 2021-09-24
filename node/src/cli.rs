/*
 * @Author: dianluyuanli-wp
 * @LastEditors: dianluyuanli-wp
 * @Date: 2021-06-16 23:28:57
 * @LastEditTime: 2021-07-20 23:46:06
 */
use structopt::StructOpt;
use sc_cli::RunCmd;

#[derive(Debug, StructOpt)]
pub struct Cli {
	//	支持子命令，类似git add
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	//	打平，RunCmd下面的命令全部提到同一层
	#[structopt(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
	Key(sc_cli::KeySubcommand),
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}
