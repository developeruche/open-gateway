// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {OpenGateway, PaymentMetadata, TokenType} from "../src/OpenGateway.sol";
import {Token} from "./ERC20.sol";

contract OpenGatewayTest is Test {
    OpenGateway public openGateway;
    address public owner = genAddress("owner");
    address public recv = genAddress("recv");
    Token paymentTokenOne;
    Token paymentTokenTwo;

    function setUp() public {
        vm.deal(owner, 1000 ether);
        vm.startPrank(owner);
        paymentTokenOne = new Token(owner, "PaymentTokenOne", "PTO");
        paymentTokenTwo = new Token(owner, "PaymentTokenTwo", "PTT");
        uint40 confirmationBlockHeight = 50;
        address[] memory paymentTokens = new address[](2);
        paymentTokens[0] = address(paymentTokenOne);
        paymentTokens[1] = address(paymentTokenTwo);
        openGateway = new OpenGateway(owner, confirmationBlockHeight, paymentTokens);
        paymentTokenOne.mint(owner, 1000 ether);
        paymentTokenTwo.mint(owner, 1000 ether);
    }

    // test contract constructor
    function test_construction() public view {
        assertEq(openGateway.owner(), owner);
        assertEq(openGateway.confirmationBlockHeight(), 50);
        assertEq(openGateway.tokenWhitelist(address(paymentTokenOne)), true);
        assertEq(openGateway.tokenWhitelist(address(paymentTokenOne)), true);
    }

    // test makePayment works fine (native token) -> (payer was debited, recipient was credited, paymentId was used, payment metadata was stored)
    function test_makePaymentNativeToken() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("1");
        openGateway.makePayment{value: amount}(amount, paymentId, address(0), owner, "metadata");
        assertEq(address(openGateway).balance, 10 ether);
        (PaymentMetadata memory paymentMetadata,) = openGateway.getPayment(paymentId);

        assertEq(paymentMetadata.processed, true);
        assertEq(paymentMetadata.amount, amount);
        assertEq(paymentMetadata.paymentId, paymentId);
        assertEq(paymentMetadata.tokenAddress, address(0));
        assertEq(paymentMetadata.payer, address(owner));
    }

    // test makePayment works fine (erc20 token) -> (payer was debited, recipient was credited, paymentId was used, payment metadata was stored)
    function test_makePaymentERC20Token() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("2");
        paymentTokenOne.approve(address(openGateway), amount);
        openGateway.makePayment(amount, paymentId, address(paymentTokenOne), owner, "metadata");
        assertEq(paymentTokenOne.balanceOf(address(openGateway)), 10 ether);
        (PaymentMetadata memory paymentMetadata,) = openGateway.getPayment(paymentId);

        assertEq(paymentMetadata.processed, true);
        assertEq(paymentMetadata.amount, amount);
        assertEq(paymentMetadata.paymentId, paymentId);
        assertEq(paymentMetadata.tokenAddress, address(paymentTokenOne));
        assertEq(paymentMetadata.payer, address(owner));
    }

    // test makePayment fails if payment token is not allowed
    function test_makePaymentWithNonWhitelistedToken() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("3");
        address nonWhitelistedToken = genAddress("nonWhitelistedToken");
        vm.expectRevert("Token not whitelisted");
        openGateway.makePayment(amount, paymentId, nonWhitelistedToken, owner, "metadata");
    }

    // test makePayment fails if paymentId is already used
    function test_makePaymentWithUsedPaymentId() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("4");
        openGateway.makePayment{value: amount}(amount, paymentId, address(0), owner, "metadata");
        vm.expectRevert("Payment exists");
        openGateway.makePayment{value: amount}(amount, paymentId, address(0), owner, "metadata");
    }

    // test makePayment fails when native token amount is not exact
    function test_makePaymentWithIncorrectNativeTokenAmount() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("5");
        vm.expectRevert("Incorrect ETH amount");
        openGateway.makePayment{value: amount - 1}(amount, paymentId, address(0), owner, "metadata");
    }

    // test updateConfirmationBlockHeight works fine
    function test_updateConfirmationBlockHeight() public {
        uint40 newConfirmationBlockHeight = 100;
        openGateway.updateConfirmationBlockHeight(newConfirmationBlockHeight);
        assertEq(openGateway.confirmationBlockHeight(), newConfirmationBlockHeight);
    }

    // test only owner can update confirmation block height
    function test_onlyOwnerCanUpdateConfirmationBlockHeight() public {
        uint40 newConfirmationBlockHeight = 100;
        vm.stopPrank();
        vm.expectRevert();
        openGateway.updateConfirmationBlockHeight(newConfirmationBlockHeight);
    }

    // test whitelist token works fine
    function test_whitelistToken() public {
        address[] memory newTokens = new address[](1);
        newTokens[0] = genAddress("newToken");
        openGateway.whitelistToken(newTokens);
        assertEq(openGateway.tokenWhitelist(newTokens[0]), true);
    }

    // test only owner can whitelist token
    function test_onlyOwnerCanWhitelistToken() public {
        address[] memory newTokens = new address[](1);
        newTokens[0] = genAddress("newToken");
        vm.stopPrank();
        vm.expectRevert();
        openGateway.whitelistToken(newTokens);
    }

    // test delist token works fine
    function test_delistToken() public {
        address[] memory newTokens = new address[](1);
        newTokens[0] = address(paymentTokenOne);
        openGateway.delistToken(newTokens);
        assertEq(openGateway.tokenWhitelist(address(paymentTokenOne)), false);
    }

    // test only owner can delist token
    function test_onlyOwnerCanDelistToken() public {
        vm.stopPrank();
        address[] memory newTokens = new address[](1);
        newTokens[0] = address(paymentTokenOne);

        vm.expectRevert();
        openGateway.delistToken(newTokens);
    }

    // test withdraw native token works fine
    function test_withdrawNativeToken() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("7");
        openGateway.makePayment{value: amount}(amount, paymentId, address(0), owner, "metadata");
        uint256 contractBalanceBefore = address(openGateway).balance;
        openGateway.withdrawNative(payable(recv));
        assertEq(address(openGateway).balance, 0);
        assertEq(recv.balance, contractBalanceBefore);
    }

    // test only owner can withdraw native token
    function test_onlyOwnerCanWithdrawNativeToken() public {
        vm.stopPrank();
        vm.expectRevert();
        openGateway.withdrawNative(payable(genAddress("other")));
    }

    // test withdraw erc20 token works fine
    function test_withdrawERC20Token() public {
        uint256 amount = 10 ether;
        bytes32 paymentId = bytes32("8");
        paymentTokenOne.approve(address(openGateway), amount);
        openGateway.makePayment(amount, paymentId, address(paymentTokenOne), owner, "metadata");
        uint256 contractBalanceBefore = paymentTokenOne.balanceOf(address(openGateway));
        openGateway.withdrawERC20(address(paymentTokenOne), recv);
        assertEq(paymentTokenOne.balanceOf(address(openGateway)), 0);
        assertEq(paymentTokenOne.balanceOf(recv), contractBalanceBefore);
    }

    // test only owner can withdraw erc20 token
    function test_onlyOwnerCanWithdrawERC20Token() public {
        vm.stopPrank();
        vm.expectRevert();
        openGateway.withdrawERC20(address(paymentTokenOne), genAddress("other"));
    }
}

function genAddress(string memory si) pure returns (address) {
    return address(uint160(uint256(keccak256(abi.encodePacked(si)))));
}
