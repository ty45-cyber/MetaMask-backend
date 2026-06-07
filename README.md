# MetaMask Mission Control

> AI-Permissioned Smart Account Execution Layer powered by MetaMask Smart Accounts, Venice AI, and 1Shot Relayer.

## 🚀 Overview

MetaMask Mission Control transforms natural language into secure onchain actions.

Instead of manually signing every transaction, users describe what they want to do in plain English. Venice AI generates an execution plan, MetaMask Smart Accounts enforce granular permissions, and 1Shot handles gas abstraction for seamless execution.

The result is a safer, faster, and more intuitive way to interact with blockchain applications.

---

## 🎯 Problem

Today's wallet experience is broken:

* Users must understand complex transaction details.
* Every action requires manual approval.
* Delegating actions to AI agents is risky.
* Gas management creates unnecessary friction.
* Most AI crypto assistants cannot safely execute transactions.

We need a secure way for users to delegate limited authority to autonomous agents without giving up control.

---

## 💡 Solution

MetaMask Mission Control introduces a permissioned execution layer for autonomous onchain agents.

Users can:

* Describe goals in natural language.
* Review AI-generated execution plans.
* Grant fine-grained permissions.
* Define spending limits and expiration times.
* Execute transactions gaslessly through 1Shot.
* Audit every action through a transparent activity log.

---

## 🏗 Architecture

### Frontend

* React
* TypeScript
* MetaMask Smart Accounts Kit

### Backend

* FastAPI
* Venice AI API
* 1Shot Relayer Integration

### Smart Contracts

* PermissionRegistry
* ExecutionRouter
* ExecutionReceipt

### Infrastructure

* Ethereum Compatible Network
* ERC-7715 Permissions
* ERC-4337 Account Abstraction

---

## 🔄 User Flow

### Step 1

Connect MetaMask Smart Account.

### Step 2

Enter an intent.

Example:

> "Send 5 USDC to my savings wallet and only allow this action for 10 minutes."

### Step 3

Venice AI generates:

* Structured execution plan
* Risk assessment
* Transaction explanation

### Step 4

User reviews permission scope:

* Allowed assets
* Spending limit
* Expiration time
* Authorized contracts

### Step 5

Approve once.

### Step 6

1Shot Relayer executes the transaction.

### Step 7

Activity is recorded in the audit trail.

---

## 🔐 Permission Model

Every agent action is constrained by:

* Maximum spend amount
* Approved contracts
* Approved functions
* Time-based expiration
* User revocation rights

Users remain in control at all times.

---

## 🤖 Venice AI Integration

Venice AI powers:

### Intent Understanding

Converts natural language into structured actions.

### Execution Planning

Generates safe transaction workflows.

### Risk Analysis

Identifies potential risks before execution.

### Transaction Explanation

Provides human-readable summaries of every action.

---

## ⚡ 1Shot Integration

1Shot provides:

* Gas abstraction
* Permissionless relayer access
* Reliable transaction execution
* Scalable infrastructure

Users can execute transactions without worrying about gas management.

---

## 🦊 MetaMask Smart Accounts Integration

Mission Control leverages:

* Smart Accounts
* ERC-7715 Advanced Permissions
* Account Abstraction
* Delegated Execution

This creates a secure foundation for agent-driven experiences.

---

## 📋 Smart Contracts

### PermissionRegistry

Stores:

* Permission scopes
* Spending limits
* Expiration windows
* Agent authorizations

### ExecutionRouter

Validates:

* Permission scope
* Spending limits
* Authorized targets

Then routes execution.

### ExecutionReceipt

Maintains:

* Transaction history
* Execution metadata
* Audit records

---

## 🌟 Key Features

✅ Natural language transaction execution

✅ AI-generated execution plans

✅ Fine-grained wallet permissions

✅ Gasless user experience

✅ Onchain audit trail

✅ Revocable agent access

✅ Smart Account native

✅ Account Abstraction ready

---

## 🎬 Demo Scenario

1. Connect wallet.

2. Create Smart Account.

3. Enter:

   > "Send 5 USDC to my vault and expire permission after 10 minutes."

4. Review permission card.

5. Approve.

6. Execute through 1Shot.

7. View audit trail and confirmation.

---

## 🔮 Future Roadmap

### Phase 1

Single-agent execution.

### Phase 2

Multi-agent coordination.

### Phase 3

Recurring permission schedules.

### Phase 4

Cross-chain execution.

### Phase 5

Enterprise policy engines.

---

## 🏆 Hackathon Tracks

This project qualifies for:

* Best Agent
* Best A2A Coordination
* Best x402 + ERC-7710
* Best Use of Venice AI
* Best Use of 1Shot Permissionless Relayer

---

## 👥 Team

Built for the MetaMask Smart Accounts Kit × 1Shot API × Venice AI Dev Cook Off.

### Vision

To become the permissioned execution layer powering the next generation of autonomous onchain agents.

---

## License

MIT
