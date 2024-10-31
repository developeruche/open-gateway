/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  EventFragment,
  AddressLike,
  ContractRunner,
  ContractMethod,
  Listener,
} from 'ethers'
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedLogDescription,
  TypedListener,
  TypedContractMethod,
} from './common'

export type PaymentMetadataStruct = {
  amount: BigNumberish
  paymentId: BytesLike
  paymentBlock: BigNumberish
  tokenAddress: AddressLike
  tokenType: BigNumberish
  payer: AddressLike
  metadata: string
  processed: boolean
}

export type PaymentMetadataStructOutput = [
  amount: bigint,
  paymentId: string,
  paymentBlock: bigint,
  tokenAddress: string,
  tokenType: bigint,
  payer: string,
  metadata: string,
  processed: boolean
] & {
  amount: bigint
  paymentId: string
  paymentBlock: bigint
  tokenAddress: string
  tokenType: bigint
  payer: string
  metadata: string
  processed: boolean
}

export interface OpenGatewayInterface extends Interface {
  getFunction(
    nameOrSignature:
      | 'confirmationBlockHeight'
      | 'delistToken'
      | 'getPayment'
      | 'makePayment'
      | 'makePaymentWithPermit'
      | 'owner'
      | 'paused'
      | 'payments'
      | 'renounceOwnership'
      | 'tokenWhitelist'
      | 'transferOwnership'
      | 'updateConfirmationBlockHeight'
      | 'whitelistToken'
      | 'withdrawERC20'
      | 'withdrawNative'
  ): FunctionFragment

  getEvent(
    nameOrSignatureOrTopic:
      | 'NativeWithdrawn'
      | 'OwnershipTransferred'
      | 'Paused'
      | 'PaymentCompleted'
      | 'TokenDelisted'
      | 'TokenWhitelisted'
      | 'Unpaused'
      | 'UpdatedConfirmationBlockHeight'
      | 'WithdrawalFailed'
  ): EventFragment

  encodeFunctionData(functionFragment: 'confirmationBlockHeight', values?: undefined): string
  encodeFunctionData(functionFragment: 'delistToken', values: [AddressLike[]]): string
  encodeFunctionData(functionFragment: 'getPayment', values: [BytesLike]): string
  encodeFunctionData(
    functionFragment: 'makePayment',
    values: [BigNumberish, BytesLike, AddressLike, AddressLike, string]
  ): string
  encodeFunctionData(
    functionFragment: 'makePaymentWithPermit',
    values: [
      BigNumberish,
      BytesLike,
      AddressLike,
      AddressLike,
      BigNumberish,
      BigNumberish,
      BytesLike,
      BytesLike,
      string
    ]
  ): string
  encodeFunctionData(functionFragment: 'owner', values?: undefined): string
  encodeFunctionData(functionFragment: 'paused', values?: undefined): string
  encodeFunctionData(functionFragment: 'payments', values: [BytesLike]): string
  encodeFunctionData(functionFragment: 'renounceOwnership', values?: undefined): string
  encodeFunctionData(functionFragment: 'tokenWhitelist', values: [AddressLike]): string
  encodeFunctionData(functionFragment: 'transferOwnership', values: [AddressLike]): string
  encodeFunctionData(functionFragment: 'updateConfirmationBlockHeight', values: [BigNumberish]): string
  encodeFunctionData(functionFragment: 'whitelistToken', values: [AddressLike[]]): string
  encodeFunctionData(functionFragment: 'withdrawERC20', values: [AddressLike, AddressLike]): string
  encodeFunctionData(functionFragment: 'withdrawNative', values: [AddressLike]): string

  decodeFunctionResult(functionFragment: 'confirmationBlockHeight', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'delistToken', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'getPayment', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'makePayment', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'makePaymentWithPermit', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'owner', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'paused', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'payments', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'renounceOwnership', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'tokenWhitelist', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'transferOwnership', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'updateConfirmationBlockHeight', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'whitelistToken', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'withdrawERC20', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'withdrawNative', data: BytesLike): Result
}

export namespace NativeWithdrawnEvent {
  export type InputTuple = [recipient: AddressLike, balance: BigNumberish]
  export type OutputTuple = [recipient: string, balance: bigint]
  export interface OutputObject {
    recipient: string
    balance: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace OwnershipTransferredEvent {
  export type InputTuple = [previousOwner: AddressLike, newOwner: AddressLike]
  export type OutputTuple = [previousOwner: string, newOwner: string]
  export interface OutputObject {
    previousOwner: string
    newOwner: string
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace PausedEvent {
  export type InputTuple = [account: AddressLike]
  export type OutputTuple = [account: string]
  export interface OutputObject {
    account: string
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace PaymentCompletedEvent {
  export type InputTuple = [paymentId: BytesLike, payer: AddressLike, tokenAddress: AddressLike, amount: BigNumberish]
  export type OutputTuple = [paymentId: string, payer: string, tokenAddress: string, amount: bigint]
  export interface OutputObject {
    paymentId: string
    payer: string
    tokenAddress: string
    amount: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace TokenDelistedEvent {
  export type InputTuple = [token: AddressLike[], timestamp: BigNumberish]
  export type OutputTuple = [token: string[], timestamp: bigint]
  export interface OutputObject {
    token: string[]
    timestamp: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace TokenWhitelistedEvent {
  export type InputTuple = [token: AddressLike[], timestamp: BigNumberish]
  export type OutputTuple = [token: string[], timestamp: bigint]
  export interface OutputObject {
    token: string[]
    timestamp: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace UnpausedEvent {
  export type InputTuple = [account: AddressLike]
  export type OutputTuple = [account: string]
  export interface OutputObject {
    account: string
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace UpdatedConfirmationBlockHeightEvent {
  export type InputTuple = [newConfirmationBlockHeight: BigNumberish]
  export type OutputTuple = [newConfirmationBlockHeight: bigint]
  export interface OutputObject {
    newConfirmationBlockHeight: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export namespace WithdrawalFailedEvent {
  export type InputTuple = [recipient: AddressLike, balance: BigNumberish]
  export type OutputTuple = [recipient: string, balance: bigint]
  export interface OutputObject {
    recipient: string
    balance: bigint
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export interface OpenGateway extends BaseContract {
  connect(runner?: ContractRunner | null): OpenGateway
  waitForDeployment(): Promise<this>

  interface: OpenGatewayInterface

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>

  on<TCEvent extends TypedContractEvent>(event: TCEvent, listener: TypedListener<TCEvent>): Promise<this>
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>

  once<TCEvent extends TypedContractEvent>(event: TCEvent, listener: TypedListener<TCEvent>): Promise<this>
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>

  listeners<TCEvent extends TypedContractEvent>(event: TCEvent): Promise<Array<TypedListener<TCEvent>>>
  listeners(eventName?: string): Promise<Array<Listener>>
  removeAllListeners<TCEvent extends TypedContractEvent>(event?: TCEvent): Promise<this>

  confirmationBlockHeight: TypedContractMethod<[], [bigint], 'view'>

  delistToken: TypedContractMethod<[tokens: AddressLike[]], [void], 'nonpayable'>

  getPayment: TypedContractMethod<
    [paymentId: BytesLike],
    [
      [PaymentMetadataStructOutput, bigint] & {
        paymentMetadata: PaymentMetadataStructOutput
        currentBlock: bigint
      }
    ],
    'view'
  >

  makePayment: TypedContractMethod<
    [amount: BigNumberish, paymentId: BytesLike, tokenAddress: AddressLike, payer: AddressLike, metadata: string],
    [void],
    'payable'
  >

  makePaymentWithPermit: TypedContractMethod<
    [
      amount: BigNumberish,
      paymentId: BytesLike,
      tokenAddress: AddressLike,
      payer: AddressLike,
      deadline: BigNumberish,
      v: BigNumberish,
      r: BytesLike,
      s: BytesLike,
      metadata: string
    ],
    [void],
    'nonpayable'
  >

  owner: TypedContractMethod<[], [string], 'view'>

  paused: TypedContractMethod<[], [boolean], 'view'>

  payments: TypedContractMethod<
    [arg0: BytesLike],
    [
      [bigint, string, bigint, string, bigint, string, string, boolean] & {
        amount: bigint
        paymentId: string
        paymentBlock: bigint
        tokenAddress: string
        tokenType: bigint
        payer: string
        metadata: string
        processed: boolean
      }
    ],
    'view'
  >

  renounceOwnership: TypedContractMethod<[], [void], 'nonpayable'>

  tokenWhitelist: TypedContractMethod<[arg0: AddressLike], [boolean], 'view'>

  transferOwnership: TypedContractMethod<[newOwner: AddressLike], [void], 'nonpayable'>

  updateConfirmationBlockHeight: TypedContractMethod<[newConfirmationBlockHeight: BigNumberish], [void], 'nonpayable'>

  whitelistToken: TypedContractMethod<[tokens: AddressLike[]], [void], 'nonpayable'>

  withdrawERC20: TypedContractMethod<[token: AddressLike, recipient: AddressLike], [boolean], 'nonpayable'>

  withdrawNative: TypedContractMethod<[recipient: AddressLike], [boolean], 'nonpayable'>

  getFunction<T extends ContractMethod = ContractMethod>(key: string | FunctionFragment): T

  getFunction(nameOrSignature: 'confirmationBlockHeight'): TypedContractMethod<[], [bigint], 'view'>
  getFunction(nameOrSignature: 'delistToken'): TypedContractMethod<[tokens: AddressLike[]], [void], 'nonpayable'>
  getFunction(nameOrSignature: 'getPayment'): TypedContractMethod<
    [paymentId: BytesLike],
    [
      [PaymentMetadataStructOutput, bigint] & {
        paymentMetadata: PaymentMetadataStructOutput
        currentBlock: bigint
      }
    ],
    'view'
  >
  getFunction(
    nameOrSignature: 'makePayment'
  ): TypedContractMethod<
    [amount: BigNumberish, paymentId: BytesLike, tokenAddress: AddressLike, payer: AddressLike, metadata: string],
    [void],
    'payable'
  >
  getFunction(
    nameOrSignature: 'makePaymentWithPermit'
  ): TypedContractMethod<
    [
      amount: BigNumberish,
      paymentId: BytesLike,
      tokenAddress: AddressLike,
      payer: AddressLike,
      deadline: BigNumberish,
      v: BigNumberish,
      r: BytesLike,
      s: BytesLike,
      metadata: string
    ],
    [void],
    'nonpayable'
  >
  getFunction(nameOrSignature: 'owner'): TypedContractMethod<[], [string], 'view'>
  getFunction(nameOrSignature: 'paused'): TypedContractMethod<[], [boolean], 'view'>
  getFunction(nameOrSignature: 'payments'): TypedContractMethod<
    [arg0: BytesLike],
    [
      [bigint, string, bigint, string, bigint, string, string, boolean] & {
        amount: bigint
        paymentId: string
        paymentBlock: bigint
        tokenAddress: string
        tokenType: bigint
        payer: string
        metadata: string
        processed: boolean
      }
    ],
    'view'
  >
  getFunction(nameOrSignature: 'renounceOwnership'): TypedContractMethod<[], [void], 'nonpayable'>
  getFunction(nameOrSignature: 'tokenWhitelist'): TypedContractMethod<[arg0: AddressLike], [boolean], 'view'>
  getFunction(nameOrSignature: 'transferOwnership'): TypedContractMethod<[newOwner: AddressLike], [void], 'nonpayable'>
  getFunction(
    nameOrSignature: 'updateConfirmationBlockHeight'
  ): TypedContractMethod<[newConfirmationBlockHeight: BigNumberish], [void], 'nonpayable'>
  getFunction(nameOrSignature: 'whitelistToken'): TypedContractMethod<[tokens: AddressLike[]], [void], 'nonpayable'>
  getFunction(
    nameOrSignature: 'withdrawERC20'
  ): TypedContractMethod<[token: AddressLike, recipient: AddressLike], [boolean], 'nonpayable'>
  getFunction(nameOrSignature: 'withdrawNative'): TypedContractMethod<[recipient: AddressLike], [boolean], 'nonpayable'>

  getEvent(
    key: 'NativeWithdrawn'
  ): TypedContractEvent<
    NativeWithdrawnEvent.InputTuple,
    NativeWithdrawnEvent.OutputTuple,
    NativeWithdrawnEvent.OutputObject
  >
  getEvent(
    key: 'OwnershipTransferred'
  ): TypedContractEvent<
    OwnershipTransferredEvent.InputTuple,
    OwnershipTransferredEvent.OutputTuple,
    OwnershipTransferredEvent.OutputObject
  >
  getEvent(key: 'Paused'): TypedContractEvent<PausedEvent.InputTuple, PausedEvent.OutputTuple, PausedEvent.OutputObject>
  getEvent(
    key: 'PaymentCompleted'
  ): TypedContractEvent<
    PaymentCompletedEvent.InputTuple,
    PaymentCompletedEvent.OutputTuple,
    PaymentCompletedEvent.OutputObject
  >
  getEvent(
    key: 'TokenDelisted'
  ): TypedContractEvent<TokenDelistedEvent.InputTuple, TokenDelistedEvent.OutputTuple, TokenDelistedEvent.OutputObject>
  getEvent(
    key: 'TokenWhitelisted'
  ): TypedContractEvent<
    TokenWhitelistedEvent.InputTuple,
    TokenWhitelistedEvent.OutputTuple,
    TokenWhitelistedEvent.OutputObject
  >
  getEvent(
    key: 'Unpaused'
  ): TypedContractEvent<UnpausedEvent.InputTuple, UnpausedEvent.OutputTuple, UnpausedEvent.OutputObject>
  getEvent(
    key: 'UpdatedConfirmationBlockHeight'
  ): TypedContractEvent<
    UpdatedConfirmationBlockHeightEvent.InputTuple,
    UpdatedConfirmationBlockHeightEvent.OutputTuple,
    UpdatedConfirmationBlockHeightEvent.OutputObject
  >
  getEvent(
    key: 'WithdrawalFailed'
  ): TypedContractEvent<
    WithdrawalFailedEvent.InputTuple,
    WithdrawalFailedEvent.OutputTuple,
    WithdrawalFailedEvent.OutputObject
  >

  filters: {
    'NativeWithdrawn(address,uint256)': TypedContractEvent<
      NativeWithdrawnEvent.InputTuple,
      NativeWithdrawnEvent.OutputTuple,
      NativeWithdrawnEvent.OutputObject
    >
    NativeWithdrawn: TypedContractEvent<
      NativeWithdrawnEvent.InputTuple,
      NativeWithdrawnEvent.OutputTuple,
      NativeWithdrawnEvent.OutputObject
    >

    'OwnershipTransferred(address,address)': TypedContractEvent<
      OwnershipTransferredEvent.InputTuple,
      OwnershipTransferredEvent.OutputTuple,
      OwnershipTransferredEvent.OutputObject
    >
    OwnershipTransferred: TypedContractEvent<
      OwnershipTransferredEvent.InputTuple,
      OwnershipTransferredEvent.OutputTuple,
      OwnershipTransferredEvent.OutputObject
    >

    'Paused(address)': TypedContractEvent<PausedEvent.InputTuple, PausedEvent.OutputTuple, PausedEvent.OutputObject>
    Paused: TypedContractEvent<PausedEvent.InputTuple, PausedEvent.OutputTuple, PausedEvent.OutputObject>

    'PaymentCompleted(bytes32,address,address,uint256)': TypedContractEvent<
      PaymentCompletedEvent.InputTuple,
      PaymentCompletedEvent.OutputTuple,
      PaymentCompletedEvent.OutputObject
    >
    PaymentCompleted: TypedContractEvent<
      PaymentCompletedEvent.InputTuple,
      PaymentCompletedEvent.OutputTuple,
      PaymentCompletedEvent.OutputObject
    >

    'TokenDelisted(address[],uint256)': TypedContractEvent<
      TokenDelistedEvent.InputTuple,
      TokenDelistedEvent.OutputTuple,
      TokenDelistedEvent.OutputObject
    >
    TokenDelisted: TypedContractEvent<
      TokenDelistedEvent.InputTuple,
      TokenDelistedEvent.OutputTuple,
      TokenDelistedEvent.OutputObject
    >

    'TokenWhitelisted(address[],uint256)': TypedContractEvent<
      TokenWhitelistedEvent.InputTuple,
      TokenWhitelistedEvent.OutputTuple,
      TokenWhitelistedEvent.OutputObject
    >
    TokenWhitelisted: TypedContractEvent<
      TokenWhitelistedEvent.InputTuple,
      TokenWhitelistedEvent.OutputTuple,
      TokenWhitelistedEvent.OutputObject
    >

    'Unpaused(address)': TypedContractEvent<
      UnpausedEvent.InputTuple,
      UnpausedEvent.OutputTuple,
      UnpausedEvent.OutputObject
    >
    Unpaused: TypedContractEvent<UnpausedEvent.InputTuple, UnpausedEvent.OutputTuple, UnpausedEvent.OutputObject>

    'UpdatedConfirmationBlockHeight(uint40)': TypedContractEvent<
      UpdatedConfirmationBlockHeightEvent.InputTuple,
      UpdatedConfirmationBlockHeightEvent.OutputTuple,
      UpdatedConfirmationBlockHeightEvent.OutputObject
    >
    UpdatedConfirmationBlockHeight: TypedContractEvent<
      UpdatedConfirmationBlockHeightEvent.InputTuple,
      UpdatedConfirmationBlockHeightEvent.OutputTuple,
      UpdatedConfirmationBlockHeightEvent.OutputObject
    >

    'WithdrawalFailed(address,uint256)': TypedContractEvent<
      WithdrawalFailedEvent.InputTuple,
      WithdrawalFailedEvent.OutputTuple,
      WithdrawalFailedEvent.OutputObject
    >
    WithdrawalFailed: TypedContractEvent<
      WithdrawalFailedEvent.InputTuple,
      WithdrawalFailedEvent.OutputTuple,
      WithdrawalFailedEvent.OutputObject
    >
  }
}
