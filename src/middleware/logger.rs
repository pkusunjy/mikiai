use log4rs;

pub fn setup_logging() {
    log4rs::init_file("conf/log.yaml", Default::default()).unwrap()
}
