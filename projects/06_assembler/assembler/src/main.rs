extern crate assembler;

use assembler::config::Config;

fn main() {
    let config = Config::from_args();
    assembler::run(config);
}
