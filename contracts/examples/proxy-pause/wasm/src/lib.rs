// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Number of endpoints: 8

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    proxy_pause
    (
        addContracts
        removeContracts
        addOwners
        removeOwners
        pause
        unpause
        owners
        contracts
    )
}

elrond_wasm_node::wasm_empty_callback! {}
