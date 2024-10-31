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

export interface OpenGatewayFactoryInterface extends Interface {
  getFunction(nameOrSignature: 'computeAddress' | 'deployGateway'): FunctionFragment

  getEvent(nameOrSignatureOrTopic: 'GatewayDeployed'): EventFragment

  encodeFunctionData(
    functionFragment: 'computeAddress',
    values: [BytesLike, AddressLike, BigNumberish, AddressLike[]]
  ): string
  encodeFunctionData(
    functionFragment: 'deployGateway',
    values: [BytesLike, AddressLike, BigNumberish, AddressLike[]]
  ): string

  decodeFunctionResult(functionFragment: 'computeAddress', data: BytesLike): Result
  decodeFunctionResult(functionFragment: 'deployGateway', data: BytesLike): Result
}

export namespace GatewayDeployedEvent {
  export type InputTuple = [
    gateway: AddressLike,
    owner: AddressLike,
    confirmationBlockHeight: BigNumberish,
    tokens: AddressLike[],
    salt: BytesLike
  ]
  export type OutputTuple = [
    gateway: string,
    owner: string,
    confirmationBlockHeight: bigint,
    tokens: string[],
    salt: string
  ]
  export interface OutputObject {
    gateway: string
    owner: string
    confirmationBlockHeight: bigint
    tokens: string[]
    salt: string
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>
  export type Filter = TypedDeferredTopicFilter<Event>
  export type Log = TypedEventLog<Event>
  export type LogDescription = TypedLogDescription<Event>
}

export interface OpenGatewayFactory extends BaseContract {
  connect(runner?: ContractRunner | null): OpenGatewayFactory
  waitForDeployment(): Promise<this>

  interface: OpenGatewayFactoryInterface

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

  computeAddress: TypedContractMethod<
    [salt: BytesLike, owner: AddressLike, confirmationBlockHeight: BigNumberish, tokens: AddressLike[]],
    [string],
    'view'
  >

  deployGateway: TypedContractMethod<
    [salt: BytesLike, owner: AddressLike, confirmationBlockHeight: BigNumberish, tokens: AddressLike[]],
    [string],
    'nonpayable'
  >

  getFunction<T extends ContractMethod = ContractMethod>(key: string | FunctionFragment): T

  getFunction(
    nameOrSignature: 'computeAddress'
  ): TypedContractMethod<
    [salt: BytesLike, owner: AddressLike, confirmationBlockHeight: BigNumberish, tokens: AddressLike[]],
    [string],
    'view'
  >
  getFunction(
    nameOrSignature: 'deployGateway'
  ): TypedContractMethod<
    [salt: BytesLike, owner: AddressLike, confirmationBlockHeight: BigNumberish, tokens: AddressLike[]],
    [string],
    'nonpayable'
  >

  getEvent(
    key: 'GatewayDeployed'
  ): TypedContractEvent<
    GatewayDeployedEvent.InputTuple,
    GatewayDeployedEvent.OutputTuple,
    GatewayDeployedEvent.OutputObject
  >

  filters: {
    'GatewayDeployed(address,address,uint40,address[],bytes32)': TypedContractEvent<
      GatewayDeployedEvent.InputTuple,
      GatewayDeployedEvent.OutputTuple,
      GatewayDeployedEvent.OutputObject
    >
    GatewayDeployed: TypedContractEvent<
      GatewayDeployedEvent.InputTuple,
      GatewayDeployedEvent.OutputTuple,
      GatewayDeployedEvent.OutputObject
    >
  }
}