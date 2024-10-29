// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {PaymentMeatdata, TokenType, IOpenGateway} from "./interfaces/IOpenGateway.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title OpenGateway
 * @dev A payment gateway contract that handles both native ETH and ERC20 token payments.
 * Implements security features including pausability, ownership, and token whitelisting.
 * @notice This contract allows for secure payment processing with configurable confirmation times
 * and whitelisted tokens.
 */
contract OpenGateway is IOpenGateway, Ownable, Pausable {
    // ============================
    // STATE VARIABLES
    // ============================

    /// @notice Number of blocks required for payment confirmation
    uint40 public confirmationBlockHeight;

    /// @notice Mapping of payment IDs to their corresponding payment metadata
    /// @dev Uses bytes32 as key for gas efficiency and uniqueness
    mapping(bytes32 => PaymentMeatdata) public payments;

    /// @notice Mapping of token addresses to their whitelist status
    /// @dev true = whitelisted, false = not whitelisted
    mapping(address => bool) public tokenWhitelist;

    // ============================
    // EVENTS
    // ============================

    /**
     * @notice Emitted when a payment is successfully completed
     * @param paymentId Unique identifier for the payment
     * @param payer Address that made the payment
     * @param tokenAddress Address of the token used (address(0) for ETH)
     * @param amount Amount of tokens/ETH paid
     */
    event PaymentCompleted(
        bytes32 indexed paymentId, address indexed payer, address indexed tokenAddress, uint256 amount
    );

    /**
     * @notice Emitted when new tokens are added to the whitelist
     * @param token Array of token addresses being whitelisted
     * @param timestamp Block timestamp when the whitelist was updated
     */
    event TokenWhitelisted(address[] token, uint256 timestamp);

    /**
     * @notice Emitted when tokens are removed from the whitelist
     * @param token Array of token addresses being delisted
     * @param timestamp Block timestamp when the tokens were delisted
     */
    event TokenDelisted(address[] token, uint256 timestamp);

    /**
     * @notice Emitted when the confirmation block height is updated
     * @param newConfirmationBlockHeight New number of blocks required for confirmation
     */
    event UpdatedConfirmationBlockHeight(uint40 newConfirmationBlockHeight);
    event NativeWithdrawn(address recipient, uint256 balance);
    event WithdrawalFailed(address recipient, uint256 balance);

    /**
     * @notice Contract constructor
     * @param initialOwner Address of the initial contract owner
     * @param _confirmationBlockHeight Initial number of blocks required for confirmation
     * @param tokens Array of initial whitelisted token addresses
     */
    constructor(address initialOwner, uint40 _confirmationBlockHeight, address[] memory tokens) Ownable(initialOwner) {
        confirmationBlockHeight = _confirmationBlockHeight;
        _updateTokenList(tokens, true);
    }

    /**
     * @notice Process a payment in either ETH or ERC20 tokens
     * @dev For ETH payments, msg.value must match amount. For ERC20, msg.value must be 0
     * @param amount Amount of tokens/ETH to transfer
     * @param paymentId Unique identifier for the payment
     * @param tokenAddress Address of the ERC20 token (address(0) for ETH)
     * @param payer Address making the payment
     * @param metadata Additional payment information
     */
    function makePayment(
        uint256 amount,
        bytes32 paymentId,
        address tokenAddress,
        address payer,
        string calldata metadata
    ) external payable whenNotPaused {
        // Verify payment ID hasn't been used before
        require(payments[paymentId].paymentId == bytes32(0), "Payment exists");

        // Determine token type and validate payment
        TokenType tokenType = tokenAddress == address(0) ? TokenType.NATIVE : TokenType.ERC20;
        if (tokenType == TokenType.NATIVE) {
            require(msg.value == amount, "Incorrect ETH amount");
        } else {
            require(msg.value == 0, "ETH not accepted for ERC20");
            transferTokens(tokenAddress, payer, amount);
        }

        // Store payment metadata
        payments[paymentId] = PaymentMeatdata({
            amount: amount,
            paymentId: paymentId,
            paymentBlock: uint40(block.timestamp),
            tokenAddress: tokenAddress,
            tokenType: tokenType,
            payer: payer,
            metadata: metadata
        });
    }

    /**
     * @notice Retrieve payment information and current block number
     * @param paymentId Unique identifier of the payment to query
     * @return paymentMetadata Struct containing payment details
     * @return currentBlock Current block number
     */
    function getPayment(bytes32 paymentId)
        external
        view
        returns (PaymentMeatdata memory paymentMetadata, uint256 currentBlock)
    {
        return (payments[paymentId], block.number);
    }

    /**
     * @notice Update the number of blocks required for payment confirmation
     * @dev Only callable by contract owner
     * @param newConfirmationBlockHeight New confirmation block height
     */
    function updateConfirmationBlockHeight(uint40 newConfirmationBlockHeight) external onlyOwner {
        confirmationBlockHeight = newConfirmationBlockHeight;
        emit UpdatedConfirmationBlockHeight(newConfirmationBlockHeight);
    }

    /**
     * @notice Add tokens to the whitelist
     * @dev Only callable by contract owner
     * @param tokens Array of token addresses to whitelist
     */
    function whitelistToken(address[] memory tokens) external onlyOwner {
        _updateTokenList(tokens, true);
        emit TokenWhitelisted(tokens, block.timestamp);
    }

    /**
     * @notice Remove tokens from the whitelist
     * @dev Only callable by contract owner
     * @param tokens Array of token addresses to delist
     */
    function delistToken(address[] memory tokens) external onlyOwner {
        _updateTokenList(tokens, false);
        emit TokenDelisted(tokens, block.timestamp);
    }

    /**
     * @notice Withdraw accumulated ERC20 tokens from the contract
     * @dev Only callable by contract owner. Allows withdrawal of specific tokens
     * @param token Array of token addresses to withdraw
     * @param recipient Address to receive the tokens
     * @return success Boolean array indicating success of each token withdrawal
     */
    function withdrawERC20(address token, address recipient) external onlyOwner returns (bool success) {
        // Input validation
        require(recipient != address(0), "Invalid recipient");
        require(token != address(0), "Invalid token");

        // Get token balance of this contract using balanceOf(address)
        bytes memory balanceData = abi.encodeWithSignature("balanceOf(address)", address(this));
        (bool balanceSuccess, bytes memory balanceResult) = token.call(balanceData);
        require(balanceSuccess && balanceResult.length >= 32, "Balance check failed");

        uint256 balance = abi.decode(balanceResult, (uint256));
        require(balance > 0, "No tokens to withdraw");

        // Transfer tokens using transfer(address,uint256)
        bytes memory transferData = abi.encodeWithSignature("transfer(address,uint256)", recipient, balance);

        (success,) = token.call(transferData);
        require(success, "Transfer failed");

        // Verify the transfer was successful by checking return value
        // Some tokens don't return a value, so we check if either there's no return data
        // or if the return data decodes to true
        require(success && (transferData.length == 0 || abi.decode(transferData, (bool))), "Transfer failed");

        return success;
    }

    /**
     * @notice Withdraw accumulated native ETH from the contract
     * @dev Only callable by contract owner
     * @param recipient Address to receive the ETH
     * @return success Boolean indicating if the withdrawal was successful
     */
    function withdrawNative(address payable recipient) external onlyOwner returns (bool success) {
        // Input validation
        require(recipient != address(0), "Invalid recipient");

        // Get contract's ETH balance
        uint256 balance = address(this).balance;
        require(balance > 0, "No ETH to withdraw");

        // Attempt to transfer ETH
        (success,) = recipient.call{value: balance}("");

        if (success) {
            emit NativeWithdrawn(recipient, balance);
        } else {
            emit WithdrawalFailed(recipient, balance);
        }
    }

    // ==============================
    // INTERNAL UTILS FUNCTIONS
    // ==============================

    /**
     * @notice Transfer ERC20 tokens from payer to contract
     * @dev Uses low-level call for broader token compatibility
     * @param tokenAddress Address of the ERC20 token contract
     * @param payer Address sending the tokens
     * @param amount Amount of tokens to transfer
     */
    function transferTokens(address tokenAddress, address payer, uint256 amount) internal {
        // Construct the data for transferFrom(address,address,uint256) call
        bytes memory data =
            abi.encodeWithSignature("transferFrom(address,address,uint256)", payer, address(this), amount);

        // Perform the low-level call
        (bool success, bytes memory result) = tokenAddress.call(data);

        // Check if the call was successful and decode result if necessary
        require(success && (result.length == 0 || abi.decode(result, (bool))), "Transfer failed");
    }

    /**
     * @notice Update whitelist status for multiple tokens
     * @dev Internal function used by both whitelist and delist operations
     * @param tokens Array of token addresses to update
     * @param status New whitelist status for the tokens
     */
    function _updateTokenList(address[] memory tokens, bool status) internal {
        for (uint256 i = 0; i < tokens.length; i++) {
            tokenWhitelist[tokens[i]] = status;
        }
    }
}
