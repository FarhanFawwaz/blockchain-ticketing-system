#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, Vec, Address
};

#[contract]
pub struct TicketContract;

#[contracttype]
#[derive(Clone)]
pub struct Ticket {
    pub id: u32,
    pub event_name: String,
    pub owner: Address,
    pub price: u32,
    pub is_used: bool,
    pub expired: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TicketCount,
    Ticket(u32),
    UserTickets(Address),
}

#[contractimpl]
impl TicketContract {

    // INIT
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TicketCount, &0u32);
    }

    // CREATE TICKET
    pub fn create_ticket(env: Env, event_name: String, price: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut count: u32 = env.storage().instance().get(&DataKey::TicketCount).unwrap();
        count += 1;

        let ticket = Ticket {
            id: count,
            event_name,
            owner: admin.clone(),
            price,
            is_used: false,
            expired: false,
        };

        env.storage().instance().set(&DataKey::Ticket(count), &ticket);
        env.storage().instance().set(&DataKey::TicketCount, &count);
    }

    // BUY TICKET (limit max 2)
    pub fn buy_ticket(env: Env, user: Address, ticket_id: u32) {
        user.require_auth();

        let mut ticket: Ticket = env.storage().instance().get(&DataKey::Ticket(ticket_id)).unwrap();

        if ticket.is_used || ticket.expired {
            panic!("Ticket invalid");
        }

        let key = DataKey::UserTickets(user.clone());
        let mut list: Vec<u32> = env.storage().instance().get(&key).unwrap_or(Vec::new(&env));

        if list.len() >= 2 {
            panic!("Max 2 tickets per user");
        }

        ticket.owner = user.clone();
        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);

        list.push_back(ticket_id);
        env.storage().instance().set(&key, &list);
    }

    // TRANSFER
    pub fn transfer_ticket(env: Env, from: Address, to: Address, ticket_id: u32) {
        from.require_auth();

        let mut ticket: Ticket = env.storage().instance().get(&DataKey::Ticket(ticket_id)).unwrap();

        if ticket.owner != from {
            panic!("Not owner");
        }

        if ticket.is_used {
            panic!("Already used");
        }

        ticket.owner = to;
        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // VALIDATE (USE TICKET)
    pub fn use_ticket(env: Env, ticket_id: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut ticket: Ticket = env.storage().instance().get(&DataKey::Ticket(ticket_id)).unwrap();

        if ticket.is_used {
            panic!("Already used");
        }

        ticket.is_used = true;
        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // EXPIRE
    pub fn expire_ticket(env: Env, ticket_id: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut ticket: Ticket = env.storage().instance().get(&DataKey::Ticket(ticket_id)).unwrap();
        ticket.expired = true;

        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // GET
    pub fn get_ticket(env: Env, ticket_id: u32) -> Ticket {
        env.storage().instance().get(&DataKey::Ticket(ticket_id)).unwrap()
    }

    pub fn get_user_tickets(env: Env, user: Address) -> Vec<u32> {
        env.storage().instance().get(&DataKey::UserTickets(user)).unwrap_or(Vec::new(&env))
    }
}