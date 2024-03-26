// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler_with_message!();

multiversx_sc_wasm_adapter::endpoints! {
    panic_message_features
    (
        init => init
        panicWithMessage => panic_with_message
        panicAfterLog => panic_after_log
        sc_panic => sc_panic
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
