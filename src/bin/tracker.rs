#[tracker::track]
struct Test {
    x: u8,
    y: u64,
}

fn main() {
    let mut t = Test {
        x: 0,
        y: 0,
        // the macro generates a new variable called
        // "tracker" which stores the changes
        tracker: 0,
    };

    t.set_x(42);

    // let's check whether the change was detected
    dbg!(t.changed(Test::x()));

    // reset t so we don't track old changes
    t.reset();

    t.set_x(42);

    // same value, so no change
    dbg!(t.changed(Test::x()));
}
