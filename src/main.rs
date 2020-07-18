#![feature(try_trait)]

mod config_man;
mod model;

fn main() {
    config_man::run_checks();
    let savga_config = config_man::get_config();

}
