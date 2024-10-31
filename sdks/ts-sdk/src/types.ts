export interface PrePaymentParams {
  amount: bigint
  tokenAddress: string
  payer: string
  metadata: string
}

export interface PaymentParams {
  paymentId: string
  amount: bigint
  tokenAddress: string
  payer: string
  metadata: string
}

export interface PaymentMetadata {
  amount: bigint
  paymentId: string
  paymentBlock: number
  tokenAddress: string
  tokenType: number // 0 for NATIVE, 1 for ERC20
  payer: string
  metadata: string
  processed: boolean // true is payment has been
}
