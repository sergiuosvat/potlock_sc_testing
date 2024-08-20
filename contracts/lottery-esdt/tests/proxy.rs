// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct LotteryProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for LotteryProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = LotteryProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        LotteryProxyMethods { wrapped_tx: tx }
    }
}

pub struct LotteryProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> LotteryProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> LotteryProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn start<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<Option<usize>>,
        Arg4: ProxyArg<Option<u64>>,
        Arg5: ProxyArg<Option<usize>>,
        Arg6: ProxyArg<Option<ManagedVec<Env::Api, u8>>>,
        Arg7: ProxyArg<Option<ManagedVec<Env::Api, ManagedAddress<Env::Api>>>>,
        Arg8: ProxyArg<OptionalValue<BigUint<Env::Api>>>,
    >(
        self,
        lottery_name: Arg0,
        token_identifier: Arg1,
        ticket_price: Arg2,
        opt_total_tickets: Arg3,
        opt_deadline: Arg4,
        opt_max_entries_per_user: Arg5,
        opt_prize_distribution: Arg6,
        opt_whitelist: Arg7,
        opt_burn_percentage: Arg8,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("start")
            .argument(&lottery_name)
            .argument(&token_identifier)
            .argument(&ticket_price)
            .argument(&opt_total_tickets)
            .argument(&opt_deadline)
            .argument(&opt_max_entries_per_user)
            .argument(&opt_prize_distribution)
            .argument(&opt_whitelist)
            .argument(&opt_burn_percentage)
            .original_result()
    }

    pub fn create_lottery_pool<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<EgldOrEsdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<Option<usize>>,
        Arg4: ProxyArg<Option<u64>>,
        Arg5: ProxyArg<Option<usize>>,
        Arg6: ProxyArg<Option<ManagedVec<Env::Api, u8>>>,
        Arg7: ProxyArg<Option<ManagedVec<Env::Api, ManagedAddress<Env::Api>>>>,
        Arg8: ProxyArg<OptionalValue<BigUint<Env::Api>>>,
    >(
        self,
        lottery_name: Arg0,
        token_identifier: Arg1,
        ticket_price: Arg2,
        opt_total_tickets: Arg3,
        opt_deadline: Arg4,
        opt_max_entries_per_user: Arg5,
        opt_prize_distribution: Arg6,
        opt_whitelist: Arg7,
        opt_burn_percentage: Arg8,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("createLotteryPool")
            .argument(&lottery_name)
            .argument(&token_identifier)
            .argument(&ticket_price)
            .argument(&opt_total_tickets)
            .argument(&opt_deadline)
            .argument(&opt_max_entries_per_user)
            .argument(&opt_prize_distribution)
            .argument(&opt_whitelist)
            .argument(&opt_burn_percentage)
            .original_result()
    }

    pub fn buy_ticket<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        lottery_name: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("buy_ticket")
            .argument(&lottery_name)
            .original_result()
    }

    pub fn determine_winner<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        lottery_name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("determine_winner")
            .argument(&lottery_name)
            .original_result()
    }

    pub fn status<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        lottery_name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Status> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("status")
            .argument(&lottery_name)
            .original_result()
    }

    pub fn lottery_info<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        lottery_name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, LotteryInfo<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLotteryInfo")
            .argument(&lottery_name)
            .original_result()
    }

    pub fn lottery_whitelist<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        lottery_name: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLotteryWhitelist")
            .argument(&lottery_name)
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Inactive,
    Running,
    Ended,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct LotteryInfo<Api>
where
    Api: ManagedTypeApi,
{
    pub token_identifier: EgldOrEsdtTokenIdentifier<Api>,
    pub ticket_price: BigUint<Api>,
    pub tickets_left: usize,
    pub deadline: u64,
    pub max_entries_per_user: usize,
    pub prize_distribution: ManagedVec<Api, u8>,
    pub prize_pool: BigUint<Api>,
}
