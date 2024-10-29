// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {OpenGatewayFactory} from "../src/OpenGatewayFactory.sol";
import {OpenGateway} from "../src/OpenGateway.sol";
import {Token} from "./ERC20.sol";

contract OpenGatewayFactoryTest is Test {
    OpenGatewayFactory public factory;
    address public owner = genAddress("owner");
    Token public paymentTokenOne;
    Token public paymentTokenTwo;
    uint40 public confirmationBlockHeight;
    address[] public paymentTokens;
    bytes32 public constant SALT = bytes32(uint256(1));

    function setUp() public {
        vm.deal(owner, 1000 ether);
        vm.startPrank(owner);

        // Deploy the factory
        factory = new OpenGatewayFactory();

        // Setup test tokens
        paymentTokenOne = new Token(owner, "PaymentTokenOne", "PTO");
        paymentTokenTwo = new Token(owner, "PaymentTokenTwo", "PTT");

        // Setup deployment parameters
        confirmationBlockHeight = 50;
        paymentTokens = new address[](2);
        paymentTokens[0] = address(paymentTokenOne);
        paymentTokens[1] = address(paymentTokenTwo);

        vm.stopPrank();
    }

    function test_computeAddress() public view {
        // Compute the address
        address computedAddress = factory.computeAddress(SALT, owner, confirmationBlockHeight, paymentTokens);

        // Address should not be zero
        assertTrue(computedAddress != address(0));

        // Computing the same address twice should yield the same result
        address computedAddress2 = factory.computeAddress(SALT, owner, confirmationBlockHeight, paymentTokens);

        assertEq(computedAddress, computedAddress2);
    }

    function test_deployGateway() public {
        // First compute the expected address
        address expectedAddress = factory.computeAddress(SALT, owner, confirmationBlockHeight, paymentTokens);

        // Deploy the gateway
        vm.prank(owner);
        address deployedAddress = factory.deployGateway(SALT, owner, confirmationBlockHeight, paymentTokens);

        // Verify the deployed address matches the computed address
        assertEq(deployedAddress, expectedAddress);

        // Verify the gateway was properly initialized
        OpenGateway gateway = OpenGateway(deployedAddress);
        assertEq(gateway.owner(), owner);
        assertEq(gateway.confirmationBlockHeight(), confirmationBlockHeight);
        assertEq(gateway.tokenWhitelist(address(paymentTokenOne)), true);
        assertEq(gateway.tokenWhitelist(address(paymentTokenTwo)), true);
    }

    function test_deployGatewayWithDifferentSalts() public {
        // Deploy with first salt
        bytes32 salt1 = bytes32(uint256(1));
        address deployment1 = factory.deployGateway(salt1, owner, confirmationBlockHeight, paymentTokens);

        // Deploy with second salt
        bytes32 salt2 = bytes32(uint256(2));
        address deployment2 = factory.deployGateway(salt2, owner, confirmationBlockHeight, paymentTokens);

        // Addresses should be different
        assertTrue(deployment1 != deployment2);
    }

    function test_deploymentEvent() public {
        vm.prank(owner);

        // Expect the GatewayDeployed event to be emitted
        vm.expectEmit(true, true, false, true);
        emit OpenGatewayFactory.GatewayDeployed(
            factory.computeAddress(SALT, owner, confirmationBlockHeight, paymentTokens),
            owner,
            confirmationBlockHeight,
            paymentTokens,
            SALT
        );

        // Deploy the gateway
        factory.deployGateway(SALT, owner, confirmationBlockHeight, paymentTokens);
    }

    function test_revertOnDuplicateDeployment() public {
        // First deployment should succeed
        vm.prank(owner);
        factory.deployGateway(SALT, owner, confirmationBlockHeight, paymentTokens);

        // Second deployment with same salt should revert
        vm.prank(owner);
        vm.expectRevert(); // CREATE2 will revert if address is already used
        factory.deployGateway(SALT, owner, confirmationBlockHeight, paymentTokens);
    }

    function test_deployWithEmptyTokenList() public {
        // Setup empty token list
        address[] memory emptyTokens = new address[](0);

        // Deploy should succeed with empty token list
        vm.prank(owner);
        address deployedAddress = factory.deployGateway(SALT, owner, confirmationBlockHeight, emptyTokens);

        // Verify deployment
        OpenGateway gateway = OpenGateway(deployedAddress);
        assertEq(gateway.owner(), owner);
        assertEq(gateway.confirmationBlockHeight(), confirmationBlockHeight);
    }
}

function genAddress(string memory si) pure returns (address) {
    return address(uint160(uint256(keccak256(abi.encodePacked(si)))));
}
