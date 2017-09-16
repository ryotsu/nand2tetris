extern crate compiler_i;

use compiler_i::config::Config;

fn main() {
    let config = Config::from_args();
    compiler_i::run(config);
}
