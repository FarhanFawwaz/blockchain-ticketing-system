#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_ticket_flow() {
    let env = Env::default();

    let admin = Address::random(&env);
    let user = Address::random(&env);

    // 🔹 INIT
    TicketContract::initialize(env.clone(), admin.clone());

    // 🔹 CREATE TICKET
    TicketContract::create_ticket(env.clone(), "Seminar Blockchain".into());

    // 🔹 CHECK TICKET
    let ticket = TicketContract::get_ticket(env.clone(), 1);
    assert_eq!(ticket.id, 1);
    assert_eq!(ticket.is_used, false);

    // 🔹 BUY
    TicketContract::buy_ticket(env.clone(), user.clone(), 1);

    let ticket_after_buy = TicketContract::get_ticket(env.clone(), 1);
    assert_eq!(ticket_after_buy.owner, user);

    // 🔹 TRANSFER
    let user2 = Address::random(&env);
    TicketContract::transfer_ticket(env.clone(), user.clone(), user2.clone(), 1);

    let ticket_after_transfer = TicketContract::get_ticket(env.clone(), 1);
    assert_eq!(ticket_after_transfer.owner, user2);

    // 🔹 USE TICKET
    TicketContract::use_ticket(env.clone(), 1);

    let ticket_after_use = TicketContract::get_ticket(env.clone(), 1);
    assert_eq!(ticket_after_use.is_used, true);
}