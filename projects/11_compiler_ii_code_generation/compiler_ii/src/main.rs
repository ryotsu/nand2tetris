extern crate compiler_ii;

use compiler_ii::config::Config;

fn main() {
    let config = Config::from_args();
    compiler_ii::run(config);
}
