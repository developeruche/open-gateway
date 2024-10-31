import { PrePaymentParams } from './types'
import { ethers } from 'ethers'

export function genPaymentId(params: PrePaymentParams, appSalt: string): string {
  let rand = generateRandomString()
  const paymentId = ethers.keccak256(ethers.toUtf8Bytes(paymentParamsToString(params) + appSalt + rand))
  return paymentId
}

function paymentParamsToString(params: PrePaymentParams): string {
  return `amount=${params.amount.toString()}, tokenAddress=${params.tokenAddress}, payer=${params.payer}, metadata=${
    params.metadata
  }`
}

function generateRandomString(length: number = 32): string {
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
  let result = ''
  for (let i = 0; i < length; i++) {
    result += chars[Math.floor(Math.random() * chars.length)]
  }
  return result
}

export let genProvider = (url: string) => {
  return new ethers.JsonRpcProvider(url)
}
