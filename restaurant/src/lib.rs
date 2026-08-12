mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            super::dine_at_restaurant();
        }

        fn serve_order() {}

        pub(super) fn take_payment() {}
    }

    fn dine_at_restaurant() {
        serving::take_payment();
    }

}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
