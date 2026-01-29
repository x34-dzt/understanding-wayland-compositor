use wayland_client::{Connection, Dispatch, EventQueue, protocol::wl_registry};

struct State;

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        _state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[{}] {} (v{})", name, interface, version);
        };
    }
}

fn main() {
    let conn = Connection::connect_to_env().expect("error: failed to open a wayland connection");
    let display = conn.display();
    let mut event_queue: EventQueue<State> = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
    println!("advertised globals");

    event_queue.roundtrip(&mut State).unwrap();
}
