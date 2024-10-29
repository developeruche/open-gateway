// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

enum TokenType {
    NATIVE,
    ERC20
}

struct PaymentMetadata {
    uint256 amount;
    bytes32 paymentId;
    uint40 paymentBlock;
    address tokenAddress;
    TokenType tokenType;
    address payer;
    string metadata;
    bool processed;
}

interface IOpenGateway {
    // Core payment functions
    function makePayment(
        uint256 amount,
        bytes32 paymentId,
        address tokenAddress,
        address payer,
        string calldata metadata
    ) external payable;

    function getPayment(bytes32 paymentId)
        external
        view
        returns (PaymentMetadata memory paymentMetadata, uint256 currentBlock);

    // Configuration functions
    function updateConfirmationBlockHeight(uint40 newConfirmationBlockHeight) external;
    function whitelistToken(address[] memory tokens) external;
    function delistToken(address[] memory tokens) external;

    // Withdrawal functions
    function withdrawERC20(address token, address recipient) external returns (bool success);
    function withdrawNative(address payable recipient) external returns (bool success);

    // View functions
    function confirmationBlockHeight() external view returns (uint40);
    function tokenWhitelist(address) external view returns (bool);
}
