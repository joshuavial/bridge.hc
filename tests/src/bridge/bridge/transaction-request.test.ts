import { assert, test } from "vitest";

import { runScenario} from '@holochain/tryorama';
import { ActionHash, AgentPubKeyB64, Record, } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import {installApp} from './utils.js';

enum TransactionRequestType {
    Send = "Send",
    Receive = "Receive"
}

interface CreateTransactionRequestInput {
    transactionRequestType: TransactionRequestType,
    counterpartyPubKey: AgentPubKeyB64,
    amount: Number,
}

interface TransactionRequest {
    spenderPubKey: AgentPubKeyB64,
    recipientPubKey: AgentPubKeyB64,
    amount: Number,
}

test('Given an agent, Alice, When Alice tries to create a transaction request to herself, Then it fails', async () => {
  await runScenario(async scenario => {

    const [aliceConductor, alice] = await installApp(scenario, null);

    const forceInit: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    const transactionList: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "transaction_requests",
      fn_name: "get_my_transaction_requests",
      payload: null,
    });
    assert.equal(Object.keys(transactionList).length, 0);

    let transactionRequestInput : CreateTransactionRequestInput = {
        transactionRequestType: TransactionRequestType.Send,
        counterpartyPubKey: (alice.agentPubKey as unknown) as AgentPubKeyB64,
        amount: 10.0,
    };
    try {
        const attemptCreateRequest: any = await aliceConductor.appAgentWs().callZome({
            role_name: "bridge",
            zome_name: "transaction_requests",
            fn_name: "create_transaction_request",
            payload: transactionRequestInput,
        });

        assert.fail();
      }
      catch (e) {
        console.log(e)
        assert.ok(1);
      }

    const transactionListAfter: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "transaction_requests",
      fn_name: "get_my_transaction_requests",
      payload: null,
    });
    assert.equal(Object.keys(transactionListAfter).length, 0);
  });
});

test('Given two agents, Alice and Bob, When Alice tries to create a transaction request to Bob, Then it creates a transaction request for Alice', async () => {
  await runScenario(async scenario => {
    const [aliceConductor, alice] = await installApp(scenario, null);
    const [bobConductor, bob] = await installApp(scenario, alice.agentPubKey);

    const forceInit: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    const transactionList: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "transaction_requests",
      fn_name: "get_my_transaction_requests",
      payload: null,
    });
    assert.equal(Object.keys(transactionList).length, 0);

    let transactionRequestInput : CreateTransactionRequestInput = {
        transactionRequestType: TransactionRequestType.Send,
        counterpartyPubKey: (bob.agentPubKey as unknown) as AgentPubKeyB64,
        amount: 10.0,
    };
    try {
        const attemptCreateRequest: any = await aliceConductor.appAgentWs().callZome({
            role_name: "bridge",
            zome_name: "transaction_requests",
            fn_name: "create_transaction_request",
            payload: transactionRequestInput,
        });
        assert.ok(1)
      }
      catch (e) {
        console.log(e)
      }

      const transactionListAliceAfter: any = await aliceConductor.appAgentWs().callZome({
        role_name: "bridge",
        zome_name: "transaction_requests",
        fn_name: "get_my_transaction_requests",
        payload: null,
      });
      assert.equal(Object.keys(transactionListAliceAfter).length, 1);

      const transactionListBobAfter: any = await bobConductor.appAgentWs().callZome({
        role_name: "bridge",
        zome_name: "transaction_requests",
        fn_name: "get_my_transaction_requests",
        payload: null,
      });
      assert.equal(Object.keys(transactionListBobAfter).length, 0);
  })
})


test.skip('Given an agent, Alice, When Alice tries to create a transaction request to herself, Then it fails (TEMPLATE)', async () => {
  await runScenario(async scenario => {

    const [aliceConductor, alice] = await installApp(scenario, null);

    const forceInit: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    const transactionList: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "query_my_transactions",
      payload: null,
    });
    assert.equal(Object.keys(transactionList).length, 0);

    let transactionRequestInput : CreateTransactionRequestInput = {
        transactionRequestType: TransactionRequestType.Send,
        counterpartyPubKey: (alice.agentPubKey as unknown) as AgentPubKeyB64,
        amount: 10.0,
    };
    try {
        const attemptCreateRequest: any = await aliceConductor.appAgentWs().callZome({
            role_name: "bridge",
            zome_name: "transaction_requests",
            fn_name: "create_transaction_request",
            payload: transactionRequestInput,
        });

        assert.fail();
      }
      catch (e) {
        console.log(e)
        assert.ok(1);
      }

    const transactionListAfter: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "transaction_requests",
      fn_name: "get_my_transaction_requests",
      payload: null,
    });
    assert.equal(Object.keys(transactionListAfter).length, 0);
  });
});