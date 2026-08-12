pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}

    fn add(x: u32, y:u32) -> u32 {
        x + y
    }
}

mod serving {
    fn take_order() {
        super::dine_at_restaurant();
    }

    fn serve_order() {
        super::super::eat_at_restaurant();
    }

    pub(super) fn take_payment() {}
}

fn dine_at_restaurant() {
    serving::take_payment();
}

