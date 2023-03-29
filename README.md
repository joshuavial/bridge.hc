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
  - list of authorities (holo_pubkey, eth_pubkey) is initially just the progenitor
  - progenitor eth address and consensus percentage provided by the properties of the dht

  - role based administration of the authority list is needed to add enough peers to reach a meaningful consensus
    - Options for roles pattern

    1. Similar to private_public_publications exercise

    2. A separate dht/cell that is used to store the assigned roles and agentpubkeys.
      - how many roles would we want (do authorities all have the same role as the progenitor once the cell is established?)

 
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


# Bridge.hc

## Environment Setup

> PREREQUISITE: set up the [holochain development environment](https://developer.holochain.org/docs/install/).

Enter the nix shell by running this in the root folder of the repository: 

```bash
nix-shell
npm install
```

**Run all the other instructions in this README from inside this nix-shell, otherwise they won't work**.

## Running 2 agents
 
```bash
npm start
```

This will create a network of 2 nodes connected to each other and their respective UIs.
It will also bring up the Holochain Playground for advanced introspection of the conductors.

## Running the backend tests

```bash
npm test
```

## Bootstrapping a network

Create a custom network of nodes connected to each other and their respective UIs with:

```bash
AGENTS=3 npm run network
```

Substitute the "3" for the number of nodes that you want to bootstrap in your network.
This will also bring up the Holochain Playground for advanced introspection of the conductors.

## Packaging

To package the web happ:
``` bash
npm run package
```

You'll have the `bridge.hc.webhapp` in `workdir`. This is what you should distribute so that the Holochain Launcher can install it.
You will also have its subcomponent `bridge.hc.happ` in the same folder`.

## Documentation

This repository is using these tools:
- [NPM Workspaces](https://docs.npmjs.com/cli/v7/using-npm/workspaces/): npm v7's built-in monorepo capabilities.
- [hc](https://github.com/holochain/holochain/tree/develop/crates/hc): Holochain CLI to easily manage Holochain development instances.
- [@holochain/tryorama](https://www.npmjs.com/package/@holochain/tryorama): test framework.
- [@holochain/client](https://www.npmjs.com/package/@holochain/client): client library to connect to Holochain from the UI.
- [@holochain-playground/cli](https://www.npmjs.com/package/@holochain-playground/cli): introspection tooling to understand what's going on in the Holochain nodes.
