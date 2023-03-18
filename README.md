# bridge.hc
POC of token sidechain from other chains to holochain

## Overview
- Soliditiy contract that receives tokens and locks them in the side chain
- Holochain DHT which acts as a sidechain for that contract
- Trusted list of authorities that
  - are managed through human / cultural processes
  - credit holochain ledger based off deposits in smart contract
  - initiate withdraws from smart contract based off actions in holochain
  - are mirrored in the DHT and smart contract (active on both chains, if an agent is added as an authority on the dht then they are added as a signer on the smart contract).


## Holochain architecture

Entries
- Transaction - countersigned - [build on guillem's stuff](https://github.com/darksoil-studio/mutual-credit/blob/main/libs/transactions/zomes/integrity/src/entry.rs) - tip of the chain validation
  - spender 
  - recipient
  - amount
  - token //contract id + short name
  - cross-chain-id (bool - 0 for sidechain internal transaction, > 0 for deposits and withdrawls) 
  - cross chain reference(s) (updated when transaction processed on other chain - txid), maybe make this a set of txids in case a transaction needs to be split (e.g. large transactions in future)
  
  - validations
    - listens for 
    - countersigned by spender and recipient
    - tip of the chain validation (reference?)
    - if cross-chain is 1 then countersigned by x% of authorities
    - if cross-chain and updated then add cross-chain refrence countersigned by x% of authorities
  
- AuthorityList 
 - list of authorities (holo_pubkey, eth_pubkey)
 - initial list provided by the properties of the dht
 
 - validations
  - at least 50% sign an update to the list
  
- ChainList
  - array of bdige_contracts (id of chain (1 for Ethereum, 2 for Optimism (or whatever) etc.) (maybe use Eth chain ids) - can use for building a testnet dht + contract address)
  
  - validations
    - (super?)majority of authorities countersigned
  
- SystemStatus
  - internal transactions - required for send and receive in dht
  - external transactions - required for transactions with on-chain flag of 1 (unless it's an update, where only the tx-id is being updated)
  - update requires x% of authority list countersigning
  
## Smart contract
- deposit (pubkey on holochain, token, amount)
- withdraw (entry id on holochain, signature-set of holochain authorities approving) - anyone can initiate (and pay gas)
- add_authority (holochain pubkey, eth pubkey) - requires (super?) majority of authorities to approve
- remove_authority (holochain pubkey, eth pubkey)
- authority_commission (inbound and outbound) default to 0.001% (or whatever) 
- pay_commissions (distribute commissions to authorities)

## Constraints
- no nfts - fungible tokens only

## Questions
- can the smart contract validate holochain signatures? If so maybe it just has an array of authority holochain pub keys
- if countersigning % are parameterized then will changing those params change the integrity zome and result in a forked dht
- is extra security on witnessing transactions in dht needed for avoiding eclipse or other attacks
- maybe build a concept of "forced transaction" supermajority of authorities can countersign to credit / debit on an agents chain? Have a validation that an agent can't create a transaction until the forced one is processed?
    - entity type forcedtransactionlist - key of agent pubkey, vec of transaction details
    - maybe too much power for authorities, but could be useful measure to recover from failure states

## One day
- maybe have two types of authorities automated and manual - certain states (e.g. large transaction, significant withdrawals in short window require human review / intervention / approval)
- authorities stake funds in smart contract which is used as collatoral for bad authorisations, design system for 000s of authorities and use staked collatoral of authorities as a throttle on how much outflow they can authorise.  


