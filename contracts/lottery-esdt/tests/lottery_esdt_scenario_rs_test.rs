use multiversx_sc_scenario::*;
use imports::{EsdtLocalRole, MxscPath, TestAddress, TestTokenIdentifier, TestSCAddress};

const OWNER_ADDRESS: TestAddress = TestAddress::new("OWNER_ADDRESS");
const FIRST_ADDRESS: TestAddress = TestAddress::new("FIRST_ADDRESS");
const SECOND_ADDRESS: TestAddress = TestAddress::new("SECOND_ADDRESS");
const THIRD_ADDRESS: TestAddress = TestAddress::new("THIRD_ADDRESS");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("lottery-esdt");
const _: MxscPath = MxscPath::new("../output/lottery-esdt.mxsc.json");
const TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("BSK-476470");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.account(OWNER_ADDRESS).nonce(1);

    blockchain
        .account(FIRST_ADDRESS)
        .esdt_balance(TOKEN_IDENTIFIER, 1000)
        .nonce(1);

    blockchain
        .account(SECOND_ADDRESS)
        .esdt_balance(TOKEN_IDENTIFIER, 1000)
        .nonce(1);

    blockchain  
        .account(THIRD_ADDRESS)
        .esdt_balance(TOKEN_IDENTIFIER, 1000)
        .nonce(1);

    blockchain.register_contract(
        "mxsc:output/lottery-esdt.mxsc.json",
        lottery_esdt::ContractBuilder,
    );

    blockchain
}

#[test]
fn buy_after_deadline_rs() {
    world().run("scenarios/buy-after-deadline.scen.json");
}

#[test]
fn buy_after_sold_out_rs() {
    world().run("scenarios/buy-after-sold-out.scen.json");
}

#[test]
fn buy_after_winner_announced_rs() {
    world().run("scenarios/buy-after-winner-announced.scen.json");
}

#[test]
fn buy_all_tickets_and_determine_winner_rs() {
    world().run("scenarios/buy-all-tickets-and-determine-winner.scen.json");
}

#[test]
fn buy_all_tickets_and_exceed_max_tickets_rs() {
    world().run("scenarios/buy-all-tickets-and-exceed-max-tickets.scen.json");
}

#[test]
fn buy_all_tickets_different_accounts_rs() {
    world().run("scenarios/buy-all-tickets-different-accounts.scen.json");
}

#[test]
fn buy_more_tickets_than_allowed_rs() {
    world().run("scenarios/buy-more-tickets-than-allowed.scen.json");
}

#[test]
fn buy_not_whitelisted_rs() {
    world().run("scenarios/buy-not-whitelisted.scen.json");
}

#[test]
fn buy_ticket_rs() {
    world().run("scenarios/buy-ticket.scen.json");
}

#[test]
fn buy_ticket_after_deadline_rs() {
    world().run("scenarios/buy-ticket-after-deadline.scen.json");
}

#[test]
fn buy_ticket_after_determined_winner_rs() {
    world().run("scenarios/buy-ticket-after-determined-winner.scen.json");
}

#[test]
fn buy_ticket_after_sold_out_rs() {
    world().run("scenarios/buy-ticket-after-sold-out.scen.json");
}

#[test]
fn buy_ticket_all_options_rs() {
    world().run("scenarios/buy-ticket-all-options.scen.json");
}

#[test]
fn buy_ticket_another_account_rs() {
    world().run("scenarios/buy-ticket-another-account.scen.json");
}

#[test]
fn buy_ticket_not_on_whitelist_rs() {
    world().run("scenarios/buy-ticket-not-on-whitelist.scen.json");
}

#[test]
fn buy_ticket_same_account_rs() {
    world().run("scenarios/buy-ticket-same-account.scen.json");
}

#[test]
fn buy_ticket_second_lottery_rs() {
    world().run("scenarios/buy-ticket-second-lottery.scen.json");
}

#[test]
fn buy_ticket_wrong_fee_rs() {
    world().run("scenarios/buy-ticket-wrong-fee.scen.json");
}

#[test]
fn buy_wrong_fee_rs() {
    world().run("scenarios/buy-wrong-fee.scen.json");
}

#[test]
#[ignore]
fn complex_prize_distribution_rs() {
    world().run("scenarios/complex-prize-distribution.scen.json");
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc_1_rs() {
    world().run("scenarios/determine-winner-different-ticket-holders-winner-acc1.scen.json");
}

#[test]
fn determine_winner_early_rs() {
    world().run("scenarios/determine-winner-early.scen.json");
}

#[test]
fn determine_winner_same_ticket_holder_rs() {
    world().run("scenarios/determine-winner-same-ticket-holder.scen.json");
}

#[test]
#[ignore = "NOT SUPPORTED YET"]
fn determine_winner_split_prize_pool_rs() {
    world().run("scenarios/determine-winner-split-prize-pool.scen.json");
}
#[test]
fn init_lottery_esdt_rs() {
    world().run("scenarios/init-lottery-esdt.scen.json");
}

#[test]
fn lottery_init_rs() {
    world().run("scenarios/lottery-init.scen.json");
}

#[test]
fn lottery_with_burn_percentage_rs() {
    world().run("scenarios/lottery-with-burn-percentage.scen.json");
}

#[test]
fn start_after_announced_winner_rs() {
    world().run("scenarios/start-after-announced-winner.scen.json");
}

#[test]
fn start_all_options_bigger_whitelist_rs() {
    world().run("scenarios/start-all-options-bigger-whitelist.scen.json");
}

#[test]
fn start_alternative_function_name_rs() {
    world().run("scenarios/start-alternative-function-name.scen.json");
}

#[test]
fn start_fixed_deadline_rs() {
    world().run("scenarios/start-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_rs() {
    world().run("scenarios/start-limited-tickets.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_rs() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_rs() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json");
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_rs() {
    world().run("scenarios/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json");
}

#[test]
fn start_lottery_twice_rs() {
    world().run("scenarios/start-lottery-twice.scen.json");
}

#[test]
fn start_second_lottery_rs() {
    world().run("scenarios/start-second-lottery.scen.json");
}

#[test]
fn start_with_all_options_rs() {
    world().run("scenarios/start-with-all-options.scen.json");
}

#[test]
fn start_with_no_options_rs() {
    world().run("scenarios/start-with-no-options.scen.json");
}

#[test]
fn wrong_start_params_rs() {
    world().run("scenarios/wrong-start-params.scen.json");
}
