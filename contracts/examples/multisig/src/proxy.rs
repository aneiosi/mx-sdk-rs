////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(clippy::all)]

use multiversx_sc::imports::*;
use crate as multisig;

pub struct MultisigProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MultisigProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MultisigProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MultisigProxyMethods { wrapped_tx: tx }
    }
}

pub struct MultisigProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, Gas> MultisigProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: CodecInto<usize>,
        Arg1: CodecInto<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        quorum: Arg0,
        board: Arg1,
    ) -> Tx<
        Env,
        From,
        (),
        (),
        Gas,
        DeployCall<Env, ()>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_deploy()
            .argument(&quorum)
            .argument(&board)
            .original_result()
    }

}
impl<Env, From, To, Gas> MultisigProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Allows the contract to receive funds even if it is marked as unpayable in the protocol. 
    pub fn deposit(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("deposit")
            .original_result()
    }

    /// Iterates through all actions and retrieves those that are still pending. 
    /// Serialized full action data: 
    /// - the action id 
    /// - the serialized action data 
    /// - (number of signers followed by) list of signer addresses. 
    pub fn get_pending_action_full_info(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<MultiValueEncoded<Env::Api, multisig::action::ActionFullInfo<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getPendingActionFullInfo")
            .original_result()
    }

    /// Returns `true` (`1`) if the user has signed the action. 
    /// Does not check whether or not the user is still a board member and the signature valid. 
    pub fn signed<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<usize>,
    >(
        self,
        user: Arg0,
        action_id: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<bool>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("signed")
            .argument(&user)
            .argument(&action_id)
            .original_result()
    }

    /// Indicates user rights. 
    /// `0` = no rights, 
    /// `1` = can propose, but not sign, 
    /// `2` = can propose and sign. 
    pub fn user_role<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<multisig::user_role::UserRole>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("userRole")
            .argument(&user)
            .original_result()
    }

    /// Lists all users that can sign actions. 
    pub fn get_all_board_members(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getAllBoardMembers")
            .original_result()
    }

    /// Lists all proposers that are not board members. 
    pub fn get_all_proposers(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getAllProposers")
            .original_result()
    }

    /// Used by board members to sign actions. 
    pub fn sign<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("sign")
            .argument(&action_id)
            .original_result()
    }

    /// Board members can withdraw their signatures if they no longer desire for the action to be executed. 
    /// Actions that are left with no valid signatures can be then deleted to free up storage. 
    pub fn unsign<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("unsign")
            .argument(&action_id)
            .original_result()
    }

    /// Clears storage pertaining to an action that is no longer supposed to be executed. 
    /// Any signatures that the action received must first be removed, via `unsign`. 
    /// Otherwise this endpoint would be prone to abuse. 
    pub fn discard_action<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("discardAction")
            .argument(&action_id)
            .original_result()
    }

    /// Minimum number of signatures needed to perform any action. 
    pub fn quorum(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getQuorum")
            .original_result()
    }

    /// Denormalized board member count. 
    /// It is kept in sync with the user list by the contract. 
    pub fn num_board_members(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getNumBoardMembers")
            .original_result()
    }

    /// Denormalized proposer count. 
    /// It is kept in sync with the user list by the contract. 
    pub fn num_proposers(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getNumProposers")
            .original_result()
    }

    /// The index of the last proposed action. 
    /// 0 means that no action was ever proposed yet. 
    pub fn get_action_last_index(
        self,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getActionLastIndex")
            .original_result()
    }

    /// Serialized action data of an action with index. 
    pub fn get_action_data<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<multisig::action::Action<Env::Api>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getActionData")
            .argument(&action_id)
            .original_result()
    }

    /// Gets addresses of all users who signed an action. 
    /// Does not check if those users are still board members or not, 
    /// so the result may contain invalid signers. 
    pub fn get_action_signers<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<ManagedVec<Env::Api, ManagedAddress<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getActionSigners")
            .argument(&action_id)
            .original_result()
    }

    /// Gets addresses of all users who signed an action and are still board members. 
    /// All these signatures are currently valid. 
    pub fn get_action_signer_count<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getActionSignerCount")
            .argument(&action_id)
            .original_result()
    }

    /// It is possible for board members to lose their role. 
    /// They are not automatically removed from all actions when doing so, 
    /// therefore the contract needs to re-check every time when actions are performed. 
    /// This function is used to validate the signers before performing an action. 
    /// It also makes it easy to check before performing an action. 
    pub fn get_action_valid_signer_count<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("getActionValidSignerCount")
            .argument(&action_id)
            .original_result()
    }

    /// Initiates board member addition process. 
    /// Can also be used to promote a proposer to board member. 
    pub fn propose_add_board_member<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        board_member_address: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeAddBoardMember")
            .argument(&board_member_address)
            .original_result()
    }

    /// Initiates proposer addition process.. 
    /// Can also be used to demote a board member to proposer. 
    pub fn propose_add_proposer<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        proposer_address: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeAddProposer")
            .argument(&proposer_address)
            .original_result()
    }

    /// Removes user regardless of whether it is a board member or proposer. 
    pub fn propose_remove_user<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
    >(
        self,
        user_address: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeRemoveUser")
            .argument(&user_address)
            .original_result()
    }

    pub fn propose_change_quorum<
        Arg0: CodecInto<usize>,
    >(
        self,
        new_quorum: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeChangeQuorum")
            .argument(&new_quorum)
            .original_result()
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call. 
    /// Can send EGLD without calling anything. 
    /// Can call smart contract endpoints directly. 
    /// Doesn't really work with builtin functions. 
    pub fn propose_transfer_execute<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<FunctionCall<Env::Api>>,
    >(
        self,
        to: Arg0,
        egld_amount: Arg1,
        function_call: Arg2,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeTransferExecute")
            .argument(&to)
            .argument(&egld_amount)
            .argument(&function_call)
            .original_result()
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call. 
    /// Can call smart contract endpoints directly. 
    /// Can use ESDTTransfer/ESDTNFTTransfer/MultiESDTTransfer to send tokens, while also optionally calling endpoints. 
    /// Works well with builtin functions. 
    /// Cannot simply send EGLD directly without calling anything. 
    pub fn propose_async_call<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<FunctionCall<Env::Api>>,
    >(
        self,
        to: Arg0,
        egld_amount: Arg1,
        function_call: Arg2,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeAsyncCall")
            .argument(&to)
            .argument(&egld_amount)
            .argument(&function_call)
            .original_result()
    }

    pub fn propose_sc_deploy_from_source<
        Arg0: CodecInto<BigUint<Env::Api>>,
        Arg1: CodecInto<ManagedAddress<Env::Api>>,
        Arg2: CodecInto<CodeMetadata>,
        Arg3: CodecInto<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        amount: Arg0,
        source: Arg1,
        code_metadata: Arg2,
        arguments: Arg3,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeSCDeployFromSource")
            .argument(&amount)
            .argument(&source)
            .argument(&code_metadata)
            .argument(&arguments)
            .original_result()
    }

    pub fn propose_sc_upgrade_from_source<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<BigUint<Env::Api>>,
        Arg2: CodecInto<ManagedAddress<Env::Api>>,
        Arg3: CodecInto<CodeMetadata>,
        Arg4: CodecInto<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        sc_address: Arg0,
        amount: Arg1,
        source: Arg2,
        code_metadata: Arg3,
        arguments: Arg4,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<usize>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("proposeSCUpgradeFromSource")
            .argument(&sc_address)
            .argument(&amount)
            .argument(&source)
            .argument(&code_metadata)
            .argument(&arguments)
            .original_result()
    }

    /// Returns `true` (`1`) if `getActionValidSignerCount >= getQuorum`. 
    pub fn quorum_reached<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<bool>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("quorumReached")
            .argument(&action_id)
            .original_result()
    }

    /// Proposers and board members use this to launch signed actions. 
    pub fn perform_action_endpoint<
        Arg0: CodecInto<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<OptionalValue<ManagedAddress<Env::Api>>>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("performAction")
            .argument(&action_id)
            .original_result()
    }

    pub fn dns_register<
        Arg0: CodecInto<ManagedAddress<Env::Api>>,
        Arg1: CodecInto<ManagedBuffer<Env::Api>>,
    >(
        self,
        dns_address: Arg0,
        name: Arg1,
    ) -> Tx<
        Env,
        From,
        To,
        (),
        Gas,
        FunctionCall<Env::Api>,
        OriginalResultMarker<()>,
    > {
        self.wrapped_tx
            .raw_call()
            .function_name("dnsRegister")
            .argument(&dns_address)
            .argument(&name)
            .original_result()
    }

}
