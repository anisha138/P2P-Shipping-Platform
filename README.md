# P2P Shipping Platform

## Project Title

**P2P Shipping Platform**

## Project Description

The **P2P Shipping Platform** is a decentralized solution that enables users to send and receive shipments through peer-to-peer (P2P) interactions. Users can create shipment requests, ship packages, and confirm deliveries. The platform facilitates the management of shipping logistics without the need for centralized shipping services.

## Project Vision

To provide a decentralized, cost-effective, and efficient alternative to traditional shipping solutions by allowing users to manage and track shipments directly through a smart contract on the blockchain.

## Key Features

- **Create Shipment Request**: Users can create a shipment request with details like the sender, receiver, amount, and description.
- **Mark Shipment as Shipped**: Once a shipment is picked up, it can be marked as shipped, allowing tracking of its transit.
- **Mark Shipment as Delivered**: Once the shipment reaches the destination, it can be marked as delivered, completing the transaction.
- **Shipment Tracking**: Users can check the status of their shipment (pending, shipped, delivered, cancelled).

## Contract Details

### Contract Address: 

**Contract Address**: CDCW5XLZ64EM2T75OJQEFNS5QJEDHTJSIATR62KKT544F4TGTJZMKR6D

- **Contract Name**: P2PShippingContract
- **Functions**:
  - `create_shipment(sender, receiver, amount, description)`: Creates a new shipment request.
  - `mark_shipped(sender, shipment_id)`: Marks a shipment as shipped.
  - `mark_delivered(receiver, shipment_id)`: Marks a shipment as delivered.
  - `get_shipment(shipment_id)`: Retrieves details of a shipment by its ID.

- **State Variables**:
  - `SHIPMENT_COUNT`: A counter for creating unique shipment IDs.
  - `Shipment`: A structure storing details for each shipment, including its status and timestamps for each milestone.

---

This contract simplifies peer-to-peer shipping interactions and is built on the Soroban SDK for efficient deployment and management of smart contracts on the blockchain.
