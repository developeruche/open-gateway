# OpenGateway
OpenGateway is a decentralized payment gateway system that facilitates secure and efficient handling of payments in native ETH or ERC20 tokens. It includes a comprehensive set of tools, including smart contracts, TypeScript SDKs, and a user interface, enabling seamless integration with blockchain applications.

## Features 
•	Smart Contracts:
	•	Secure payments in ETH and ERC20 tokens.
	•	Whitelisting and management of ERC20 tokens.
	•	Support for ERC20 permit functionality.
	•	Configurable payment confirmation block height.
	•	Pausable functionality for security.
•	SDK:
	•	Provides TypeScript-based abstractions for easy interaction with OpenGateway contracts.
	•	Includes utility functions for deploying contracts, managing payments, and generating deterministic payment IDs.
•	Indexer:
	•	Tracks payment metadata and provides real-time insights into payment activity.
•	UI:
	•	User-friendly interface for managing payments and interacting with the gateway.
	

## Project Structure 
```
OpenGateway/
├── contracts        # Solidity smart contracts for the payment gateway.
├── examples         # Example implementations demonstrating how to use the gateway.
├── indexer          # Backend service for tracking payments.
├── sdks             # TypeScript SDK for interacting with the gateway.
├── ui               # Frontend UI for managing and monitoring the gateway.
└── README.md        # Project documentation (this file).	
```

# Getting Started