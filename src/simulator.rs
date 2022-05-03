mod event_queue;

enum Value { Zero, One, X, Z }


enum NetType {
    Wire,
}

struct Net {
    typ: NetType,
    val: bool,
}

struct Simulation {
    time: u32,
    nets: Vec<Net>,
}