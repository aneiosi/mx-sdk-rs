// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Number of endpoints: 8

#![no_std]

elrond_wasm_node::external_view_wasm_endpoints! {
    multisig
    (
        getPendingActionFullInfo
        userRole
        getAllBoardMembers
        getAllProposers
        getActionData
        getActionSigners
        getActionSignerCount
        getActionValidSignerCount
    )
}

elrond_wasm_node::wasm_empty_callback! {}
