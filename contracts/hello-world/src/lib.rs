#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address};

// ShipmentStatus tracks the current state of a shipment
#[contracttype]
#[derive(Clone, PartialEq)]
pub enum ShipmentStatus {
    Pending,       // Shipment request is created
    Shipped,       // Shipment has been picked up and is in transit
    Delivered,     // Shipment has been delivered to the receiver
    Cancelled      // Shipment request has been cancelled
}

// Shipment structure to store shipment details
#[contracttype]
#[derive(Clone, PartialEq)]
pub struct Shipment {
    pub shipment_id: u64,
    pub sender: Address,
    pub receiver: Address,
    pub amount: u64,          // Shipping fee
    pub description: String,
    pub status: ShipmentStatus,
    pub created_at: u64,
    pub shipped_at: Option<u64>,
    pub delivered_at: Option<u64>,
}

// Constants
const SHIPMENT_COUNT: Symbol = symbol_short!("S_COUNT");

#[contract]
pub struct P2PShippingContract;

#[contractimpl]
impl P2PShippingContract {
    // Create a new shipment request
    pub fn create_shipment(
        env: Env, 
        sender: Address, 
        receiver: Address, 
        amount: u64, 
        description: String
    ) -> u64 {
        // Authenticate the sender
        sender.require_auth();

        // Validate input
        if amount == 0 {
            log!(&env, "Invalid shipment parameters");
            panic!("Invalid shipment parameters");
        }

        // Get current shipment count and increment
        let mut shipment_count: u64 = env.storage().instance().get(&SHIPMENT_COUNT).unwrap_or(0);
        shipment_count += 1;

        // Create new shipment request
        let shipment = Shipment {
            shipment_id: shipment_count,
            sender: sender.clone(),
            receiver: receiver.clone(),
            amount,
            description,
            status: ShipmentStatus::Pending,
            created_at: env.ledger().timestamp(),
            shipped_at: None,
            delivered_at: None,
        };

        // Store the shipment
        env.storage().instance().set(&shipment_count, &shipment);

        // Update shipment count
        env.storage().instance().set(&SHIPMENT_COUNT, &shipment_count);

        log!(&env, "Shipment request created with ID: {}", shipment_count);

        shipment_count
    }

    // Mark a shipment as shipped
    pub fn mark_shipped(env: Env, sender: Address, shipment_id: u64) -> bool {
        // Authenticate the sender
        sender.require_auth();

        // Get the shipment
        let mut shipment = Self::get_shipment(env.clone(), shipment_id);

        // Validate shipment
        if shipment.shipment_id == 0 || shipment.status != ShipmentStatus::Pending {
            log!(&env, "Shipment does not exist or is not in pending state");
            return false;
        }

        // Update shipment status
        shipment.status = ShipmentStatus::Shipped;
        shipment.shipped_at = Some(env.ledger().timestamp());

        // Store updated shipment
        env.storage().instance().set(&shipment_id, &shipment);

        log!(&env, "Shipment {} marked as shipped", shipment_id);

        true
    }

    // Mark a shipment as delivered
    pub fn mark_delivered(env: Env, receiver: Address, shipment_id: u64) -> bool {
        // Authenticate the receiver
        receiver.require_auth();

        // Get the shipment
        let mut shipment = Self::get_shipment(env.clone(), shipment_id);

        // Validate shipment
        if shipment.shipment_id == 0 || shipment.status != ShipmentStatus::Shipped {
            log!(&env, "Shipment does not exist or is not in shipped state");
            return false;
        }

        // Update shipment status
        shipment.status = ShipmentStatus::Delivered;
        shipment.delivered_at = Some(env.ledger().timestamp());

        // Store updated shipment
        env.storage().instance().set(&shipment_id, &shipment);

        log!(&env, "Shipment {} marked as delivered", shipment_id);

        true
    }

    // Get shipment details
    pub fn get_shipment(env: Env, shipment_id: u64) -> Shipment {
        env.storage().instance().get(&shipment_id).unwrap_or(Shipment {
            shipment_id: 0,
            sender: Address::from_str(&env, ""),
            receiver: Address::from_str(&env, ""),
            amount: 0,
            description: String::from_str(&env, ""),
            status: ShipmentStatus::Cancelled,
            created_at: 0,
            shipped_at: None,
            delivered_at: None,
        })
    }
}
