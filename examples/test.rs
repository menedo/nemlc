use nemlc::engine::engine::Engine;

fn main() {
    let mut handle = Engine::init();
    let config = Engine::init_config("examples/demo.neml".to_string());
    handle.start(config);
}
