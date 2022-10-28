mod front_of_house {
    // First submodule
    pub mod hosting;

    // Second submodule
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
