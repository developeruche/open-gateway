export * from './types'

import { ethers } from 'ethers'
import type { Provider, Signer } from 'ethers'
import { PaymentParams, PrePaymentParams } from './types'
import { genPaymentId } from './utils'
import {
  OpenGateway,
  OpenGateway__factory,
  OpenGatewayFactory,
  OpenGatewayFactory__factory,
} from './opengateway-contract-types'
import { PaymentMetadataStructOutput } from './opengateway-contract-types/OpenGateway'

/** Constant representing native token (ETH) address */
export const NATIVE = ethers.ZeroAddress

/**
 * OpenGatewaySdk provides a wrapper around the OpenGateway smart contracts
 * facilitating easy interaction with the payment system
 */
export class OpenGatewaySdk {
  private provider: Provider
  private signer?: Signer
  private factoryContract: OpenGatewayFactory
  private confirmationBlockHeight?: number

  /**
   * Creates an instance of OpenGatewaySdk
   * @param provider - Ethereum provider instance
   * @param factoryAddress - Address of the OpenGatewayFactory contract
   * @param signer - Optional signer for transactions requiring signatures
   * @param confirmationBlockHeight - Optional default confirmation block height
   */
  constructor(provider: Provider, factoryAddress: string, signer?: Signer, confirmationBlockHeight?: number) {
    this.provider = provider
    this.signer = signer
    this.factoryContract = OpenGatewayFactory__factory.connect(factoryAddress, this.signer || this.provider)
    this.confirmationBlockHeight = confirmationBlockHeight
  }

  /**
   * Computes the deterministic address for a new gateway before deployment
   * @param salt - Unique salt for address generation
   * @param owner - Address that will own the gateway
   * @param confirmationBlockHeight - Number of blocks required for payment confirmation
   * @param tokens - Array of initially whitelisted token addresses
   * @returns Promise resolving to the computed address
   */
  async computeGatewayAddress(
    salt: string,
    owner: string,
    confirmationBlockHeight: number,
    tokens: string[]
  ): Promise<string> {
    return this.factoryContract.computeAddress(salt, owner, confirmationBlockHeight, tokens)
  }

  /**
   * Deploys a new OpenGateway contract
   * @param salt - Unique salt for deterministic deployment
   * @param owner - Address that will own the gateway
   * @param confirmationBlockHeight - Number of blocks required for payment confirmation
   * @param tokens - Array of initially whitelisted token addresses
   * @throws {Error} If no signer is provided
   * @returns Promise resolving to the deployment transaction
   */
  async deployGateway(
    salt: string,
    owner: string,
    confirmationBlockHeight: number,
    tokens: string[]
  ): Promise<ethers.ContractTransactionResponse> {
    if (!this.signer) throw new Error('Signer required for deployment')

    return this.factoryContract.deployGateway(salt, owner, confirmationBlockHeight, tokens)
  }

  /**
   * Gets an instance of the OpenGateway contract at a specific address
   * @param gatewayAddress - Address of the deployed gateway
   * @returns OpenGateway contract instance
   */
  getGatewayContract(gatewayAddress: string): OpenGateway {
    return OpenGateway__factory.connect(gatewayAddress, this.signer || this.provider)
  }

  /**
   * Creates payment parameters with a deterministic payment ID
   * @param params - Pre-payment parameters
   * @param appSalt - Application-specific salt for payment ID generation
   * @returns Payment parameters including generated payment ID
   */
  createPayment(params: PrePaymentParams, appSalt: string): PaymentParams {
    let paymentId = genPaymentId(params, appSalt)
    let paymentMetadata: PaymentParams = {
      paymentId: paymentId,
      amount: params.amount,
      tokenAddress: params.tokenAddress,
      payer: params.payer,
      metadata: params.metadata,
    }

    return paymentMetadata
  }

  /**
   * Makes a payment using native currency (ETH)
   * @param gatewayAddress - Address of the gateway contract
   * @param amount - Amount of ETH to send
   * @param paymentId - Unique payment identifier
   * @param payer - Address making the payment
   * @param metadata - Additional payment metadata
   * @returns Promise resolving to the payment transaction
   */
  async makeNativePayment(
    gatewayAddress: string,
    amount: bigint,
    paymentId: string,
    payer: string,
    metadata: string
  ): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)

    return gateway.makePayment(amount, paymentId, ethers.ZeroAddress, payer, metadata, { value: amount })
  }

  /**
   * Makes a payment using an ERC20 token
   * @param gatewayAddress - Address of the gateway contract
   * @param tokenAddress - Address of the ERC20 token
   * @param amount - Amount of tokens to send
   * @param paymentId - Unique payment identifier
   * @param payer - Address making the payment
   * @param metadata - Additional payment metadata
   * @returns Promise resolving to the payment transaction
   */
  async makeERC20Payment(
    gatewayAddress: string,
    tokenAddress: string,
    amount: bigint,
    paymentId: string,
    payer: string,
    metadata: string
  ): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)

    // First approve the gateway to spend tokens
    const tokenContract = new ethers.Contract(
      tokenAddress,
      ['function approve(address spender, uint256 amount) public returns (bool)'],
      this.signer
    )

    await tokenContract.approve(gatewayAddress, amount)

    return gateway.makePayment(amount, paymentId, tokenAddress, payer, metadata)
  }

  /**
   * Makes a payment using an ERC20 token with a permit signature
   * @param gatewayAddress - Address of the gateway contract
   * @param tokenAddress - Address of the ERC20 token
   * @param amount - Amount of tokens to send
   * @param paymentId - Unique payment identifier
   * @param payer - Address making the payment
   * @param deadline - Expiration time for the permit signature
   * @param v - Signature recovery value
   * @param r - Signature R value
   * @param s - Signature S value
   * @param metadata - Additional payment metadata
   * @returns Promise resolving to the payment transaction
   */
  async makeERC20PaymentWithPermit(
    gatewayAddress: string,
    tokenAddress: string,
    amount: bigint,
    paymentId: string,
    payer: string,
    deadline: bigint,
    v: number,
    r: string,
    s: string,
    metadata: string
  ): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)

    return gateway.makePaymentWithPermit(amount, paymentId, tokenAddress, payer, deadline, v, r, s, metadata)
  }

  /**
   * Gets the confirmation block height for a gateway
   * @param gatewayAddress - Address of the gateway contract
   * @param paymentId - Unique payment identifier
   * @returns Promise resolving to the confirmation block height
   */
  async getPayment(gatewayAddress: string, paymentId: string): Promise<[PaymentMetadataStructOutput, bigint]> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.getPayment(paymentId)
  }

  /**
   * Verifies if a payment has been processed and confirmed
   * @param gatewayAddress - Address of the gateway contract
   * @param paymentId - Payment identifier to verify
   * @returns Promise resolving to boolean indicating payment verification status
   */
  async verifyPayment(gatewayAddress: string, paymentId: string): Promise<boolean> {
    let [payment, blockHeight] = await this.getPayment(gatewayAddress, paymentId)
    let confirmationBlockHeight = await this.getConfirmationBlockHeight(gatewayAddress)
    if (payment.processed && payment.paymentBlock + confirmationBlockHeight > blockHeight) return true
    return false
  }

  /**
   * Whitelists tokens for a gateway
   * @param gatewayAddress - Address of the gateway contract
   * @param tokens - Array of token addresses to whitelist
   * @returns Promise resolving to the transaction receipt
   */
  async whitelistTokens(gatewayAddress: string, tokens: string[]): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.whitelistToken(tokens)
  }

  /**
   * Delists tokens for a gateway
   * @param gatewayAddress - Address of the gateway contract
   * @param tokens - Array of token addresses to delist
   * @returns Promise resolving to the transaction receipt
   */
  async delistTokens(gatewayAddress: string, tokens: string[]): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.delistToken(tokens)
  }

  /**
   * Updates the confirmation block height for a gateway
   * @param {string} gatewayAddress - Address of the gateway contract
   * @param {number} newHeight - New confirmation block height
   * @returns {Promise<ethers.ContractTransactionResponse>} The transaction response
   * @throws {Error} If caller is not the gateway owner
   */
  async updateConfirmationBlockHeight(
    gatewayAddress: string,
    newHeight: number
  ): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.updateConfirmationBlockHeight(newHeight)
  }

  /**
   * Withdraws native currency (ETH) from the gateway
   * @param {string} gatewayAddress - Address of the gateway contract
   * @param {string} recipient - Address to receive the withdrawn ETH
   * @returns {Promise<ethers.ContractTransactionResponse>} The transaction response
   * @throws {Error} If caller is not the gateway owner
   */
  async withdrawNative(gatewayAddress: string, recipient: string): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.withdrawNative(recipient)
  }

  /**
   * Withdraws ERC20 tokens from the gateway
   * @param {string} gatewayAddress - Address of the gateway contract
   * @param {string} tokenAddress - Address of the ERC20 token to withdraw
   * @param {string} recipient - Address to receive the withdrawn tokens
   * @returns {Promise<ethers.ContractTransactionResponse>} The transaction response
   * @throws {Error} If caller is not the gateway owner
   */
  async withdrawERC20(
    gatewayAddress: string,
    tokenAddress: string,
    recipient: string
  ): Promise<ethers.ContractTransactionResponse> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.withdrawERC20(tokenAddress, recipient)
  }

  /**
   * Checks if a token is whitelisted for a gateway
   * @param {string} gatewayAddress - Address of the gateway contract
   * @param {string} tokenAddress - Address of the token to check
   * @returns {Promise<boolean>} True if token is whitelisted, false otherwise
   */
  async isTokenWhitelisted(gatewayAddress: string, tokenAddress: string): Promise<boolean> {
    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.tokenWhitelist(tokenAddress)
  }

  /**
   * Gets the confirmation block height for a gateway
   * @param {string} gatewayAddress - Address of the gateway contract
   * @returns {Promise<bigint>} The confirmation block height
   * @note Returns the instance's confirmationBlockHeight if set, otherwise fetches from the contract
   */
  async getConfirmationBlockHeight(gatewayAddress: string): Promise<bigint> {
    if (this.confirmationBlockHeight) {
      return BigInt(this.confirmationBlockHeight)
    }

    const gateway = this.getGatewayContract(gatewayAddress)
    return gateway.confirmationBlockHeight()
  }
}
