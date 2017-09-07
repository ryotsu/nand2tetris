extern crate vm_ii;

use vm_ii::config::Config;

fn main() {
    let config = Config::from_args();
    vm_ii::run(config);
}
