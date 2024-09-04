/*
 * @Author: chenglong.zhang chenglong.zhang@kunlun-inc.com
 * @Date: 2024-09-03 14:11:59
 * @LastEditors: chenglong.zhang chenglong.zhang@kunlun-inc.com
 * @LastEditTime: 2024-09-04 10:01:04
 * @FilePath: /rcli/src/main.rs
 * @Description:
 */

use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
