/*
 * @Author: chenglong.zhang chenglong.zhang@kunlun-inc.com
 * @Date: 2024-09-03 18:17:13
 * @LastEditors: chenglong.zhang chenglong.zhang@kunlun-inc.com
 * @LastEditTime: 2024-09-03 18:29:00
 * @FilePath: /rcli/src/lib.rs
 * @Description:
 */
mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::process_csv;
