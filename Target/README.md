# 🎟️ Smart Ticketing System Based on Blockchain (Soroban - Stellar)

## 📖 1. Overview

The Smart Ticketing System is a blockchain-based application designed to manage event tickets digitally. This system enables users to create, purchase, transfer, and validate tickets with a high level of security.

Unlike conventional ticketing systems, this application utilizes blockchain technology, ensuring that data cannot be manipulated, remains transparent, and is protected from forgery.

This system is built using:
- Smart Contract (Rust + Soroban SDK)
- Stellar Blockchain (Testnet)
- Simple Frontend (HTML & JavaScript)
### 🎟️ Tampilan Frontend
![Frontend]<img width="1903" height="1056" alt="tampilan_simulasi_pesan_tiket" src="https://github.com/user-attachments/assets/ead8d403-d729-4f62-8701-154e6fe4ef2d" />

- QR Code for ticket validation
![QR Generate] <img width="740" height="752" alt="Screenshot 2026-04-15 115804" src="https://github.com/user-attachments/assets/72710577-fc4a-43e7-8d6f-69cc1b3a3ae7" />
![Uploading Screenshot 2026-04-15 115804.png…]()

---

## 🎯 2. Objectives

This application is developed with the following objectives:
- To understand the implementation of smart contracts on blockchain
- To apply CRUD (Create, Read, Update, Delete) operations
- To implement Role-Based Access Control (RBAC)
- To simulate a real-world digital ticketing system
- To reduce ticket fraud and scalping practices

---

## 🚀 3. Main Features

### 👨‍💼 Admin (Event Organizer)
- Initialize the contract
- Create event tickets
- Validate tickets (check-in process)
- Mark tickets as expired

### 👤 User
- Purchase tickets
- View owned tickets
- Transfer tickets to other users

### 🔍 General Features
- View ticket details
- Purchase limitation system (anti-scalping)
- Ticket status (active, used, expired)
- QR Code-based validation

---

## 🔐 4. Role-Based Access Control (RBAC)

This system implements role-based access control:

### Admin:
- Create tickets  
- Validate tickets  
- Manage ticket status  

### User:
- Purchase tickets  
- Transfer tickets  

Security is enforced through:
- Authentication (`require_auth`)
- Ownership validation before transactions

---

## 🧠 5. System Workflow

The system operates through the following process:

1. The admin creates event tickets on the blockchain  
2. Tickets are stored permanently on the blockchain  
3. Users purchase tickets  
4. Ticket ownership is transferred to the user  
5. The user receives a QR Code representing the ticket  
6. During the event:
   - The QR Code is scanned  
   - The system validates the ticket  
7. The ticket is marked as **"USED"** after validation  

---

## 🏗️ 6. Smart Contract Data Structure

### Ticket Structure:
- `id` → Ticket ID  
- `event_name` → Name of the event  
- `owner` → Ticket owner  
- `price` → Ticket price  
- `is_used` → Indicates whether the ticket has been used  
- `expired` → Indicates whether the ticket has expired  

---

## 🧪 7. Testing

Unit testing is implemented to ensure system correctness.

### Test Scenarios:
- Contract initialization  
- Ticket creation  
- Ticket purchase  
- Ticket transfer  
- Ticket validation  

