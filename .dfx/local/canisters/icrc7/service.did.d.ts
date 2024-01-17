import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export interface ApprovalArgs {
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'token_ids' : [] | [Array<bigint>],
  'created_at_time' : [] | [bigint],
  'expires_at' : [] | [bigint],
  'spender' : Account,
}
export type ApprovalError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'Unauthorized' : Array<bigint> } |
  { 'TooOld' : null };
export interface Icrc7CollectionMetadata {
  'icrc7_supply_cap' : [] | [bigint],
  'icrc7_description' : [] | [string],
  'icrc7_total_supply' : bigint,
  'icrc7_royalty_recipient' : [] | [Account],
  'icrc7_royalties' : [] | [number],
  'icrc7_symbol' : string,
  'icrc7_image' : [] | [string],
  'icrc7_name' : string,
}
export interface InitArgs {
  'icrc7_supply_cap' : [] | [bigint],
  'icrc7_description' : [] | [string],
  'tx_window' : [] | [bigint],
  'permitted_drift' : [] | [bigint],
  'icrc7_royalty_recipient' : [] | [Account],
  'minting_authority' : [] | [Account],
  'icrc7_royalties' : [] | [number],
  'icrc7_symbol' : string,
  'icrc7_image' : [] | [string],
  'icrc7_name' : string,
}
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export interface MintArgs {
  'to' : [] | [Account],
  'memo' : [] | [Uint8Array | number[]],
  'subaccount' : [] | [Uint8Array | number[]],
  'is_atomic' : [] | [boolean],
  'token_description' : [] | [string],
  'token_ids' : Array<bigint>,
  'image' : [] | [string],
  'token_name' : string,
}
export type MintError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'SupplyCapReached' : { 'cap' : bigint } } |
  { 'TokenIdExist' : { 'token_ids' : Array<bigint> } } |
  { 'Unauthorized' : { 'minting_authority' : [] | [Account] } };
export type Result = { 'Ok' : bigint } |
  { 'Err' : ApprovalError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : MintError };
export type Result_2 = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export interface TransferArgs {
  'to' : Account,
  'spender_subaccount' : [] | [Uint8Array | number[]],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'is_atomic' : [] | [boolean],
  'token_ids' : Array<bigint>,
  'created_at_time' : [] | [bigint],
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'Unauthorized' : { 'token_ids' : Array<bigint> } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null };
export interface _SERVICE {
  'icrc7_approve' : ActorMethod<[ApprovalArgs], Result>,
  'icrc7_balance_of' : ActorMethod<[Account], bigint>,
  'icrc7_collection_metadata' : ActorMethod<[], Icrc7CollectionMetadata>,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_image' : ActorMethod<[], [] | [string]>,
  'icrc7_metadata' : ActorMethod<[bigint], Array<[string, MetadataValue]>>,
  'icrc7_mint' : ActorMethod<[MintArgs], Result_1>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<[bigint], Account>,
  'icrc7_royalties' : ActorMethod<[], [] | [number]>,
  'icrc7_royalty_recipient' : ActorMethod<[], [] | [Account]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_tokens_of' : ActorMethod<[Account], Array<bigint>>,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<[TransferArgs], Result_2>,
}
