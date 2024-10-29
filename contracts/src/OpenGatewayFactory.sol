// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./OpenGateway.sol";

/**
 * @title OpenGatewayFactory
 * @dev Factory contract for deploying OpenGateway contracts using CREATE2
 * @notice This contract allows deterministic deployment of OpenGateway instances
 */
contract OpenGatewayFactory {
    // ============================
    // EVENTS
    // ============================

    event GatewayDeployed(
        address indexed gateway, address indexed owner, uint40 confirmationBlockHeight, address[] tokens, bytes32 salt
    );

    /**
     * @notice Computes the address where a contract will be deployed using CREATE2
     * @param salt Unique value for deterministic address generation
     * @param owner Address that will own the deployed gateway
     * @param confirmationBlockHeight Initial confirmation block height
     * @param tokens Initial whitelisted tokens
     * @return The address where the contract will be deployed
     */
    function computeAddress(bytes32 salt, address owner, uint40 confirmationBlockHeight, address[] calldata tokens)
        public
        view
        returns (address)
    {
        bytes32 hash = keccak256(
            abi.encodePacked(
                bytes1(0xff),
                address(this),
                salt,
                keccak256(
                    abi.encodePacked(type(OpenGateway).creationCode, abi.encode(owner, confirmationBlockHeight, tokens))
                )
            )
        );
        return address(uint160(uint256(hash)));
    }

    /**
     * @notice Deploys a new OpenGateway contract using CREATE2
     * @param salt Unique value for deterministic address generation
     * @param owner Address that will own the deployed gateway
     * @param confirmationBlockHeight Initial confirmation block height
     * @param tokens Initial whitelisted tokens
     * @return The address of the deployed contract
     */
    function deployGateway(bytes32 salt, address owner, uint40 confirmationBlockHeight, address[] calldata tokens)
        external
        returns (address)
    {
        // Deploy the contract using CREATE2
        OpenGateway gateway = new OpenGateway{salt: salt}(owner, confirmationBlockHeight, tokens);

        emit GatewayDeployed(address(gateway), owner, confirmationBlockHeight, tokens, salt);

        return address(gateway);
    }
}
