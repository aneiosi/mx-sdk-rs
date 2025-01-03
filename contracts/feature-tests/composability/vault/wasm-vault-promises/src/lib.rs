// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    vault
    (
        init => init
        echo_arguments => echo_arguments
        echo_arguments_without_storage => echo_arguments_without_storage
        echo_caller => echo_caller
        accept_funds => accept_funds
        accept_funds_echo_payment => accept_funds_echo_payment
        accept_funds_single_esdt_transfer => accept_funds_single_esdt_transfer
        reject_funds => reject_funds
        retrieve_funds_with_transfer_exec => retrieve_funds_with_transfer_exec
        retrieve_funds => retrieve_funds
        retrieve_funds_egld_or_single_esdt => retrieve_funds_egld_or_single_esdt
        retrieve_funds_multi_esdt => retrieve_funds_multi_esdt
        retrieve_multi_funds_async => retrieve_multi_funds_async
        burn_and_create_retrieve_async => burn_and_create_retrieve_async
        explicit_panic => explicit_panic
        get_owner_address => get_owner_address
        call_counts => call_counts
        num_called_retrieve_funds_promises => num_called_retrieve_funds_promises
        num_async_calls_sent_from_child => num_async_calls_sent_from_child
        retrieve_funds_promises => retrieve_funds_promises
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
