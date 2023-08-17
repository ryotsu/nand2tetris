use vm_i::config::Config;

fn main() {
    let config = Config::from_args();
    vm_i::run(config);
}
