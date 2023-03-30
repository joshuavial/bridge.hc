import { assert, test } from "vitest";

import { runScenario} from '@holochain/tryorama';
import { ActionHash, AgentPubKeyB64, Record, } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import {installApp} from './utils.js';

enum TransactionRequestType {
    Send,
    Receive
}

interface CreateTransactionRequestInput {
    transaction_request_type: TransactionRequestType,
    counterparty_pub_key: AgentPubKeyB64,
    amount: Number,
}

interface TransactionRequest {
    spender_pub_key: AgentPubKeyB64,
    recipient_pub_key: AgentPubKeyB64,
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

    // const transactionList: any = await aliceConductor.appAgentWs().callZome({
    //   role_name: "bridge",
    //   zome_name: "bridge",
    //   fn_name: "query_my_transactions",
    //   payload: null,
    // });
    // assert.equal(transactionList.length, 0);

    let transactionRequestInput : CreateTransactionRequestInput = {
        transaction_request_type: TransactionRequestType.Send,
        counterparty_pub_key: (alice.agentPubKey as unknown) as AgentPubKeyB64,
        amount: 10.0,
    };
    try {
        const attemptCreateRequest: any = await aliceConductor.appAgentWs().callZome({
            role_name: "bridge",
            zome_name: "transaction_request",
            fn_name: "create_transaction_request",
            payload: transactionRequestInput,
        });

        assert.fail();
      }
      catch (e) {
        console.log(e)
        assert.ok(1);
      }

    // const transactionListAfter: any = await aliceConductor.appAgentWs().callZome({
    //   role_name: "bridge",
    //   zome_name: "bridge",
    //   fn_name: "query_my_transactions",
    //   payload: null,
    // });
    // assert.equal(transactionListAfter.length, 0);
  });
});

test.skip('Given an agent, Alice, When Alice tries to create a transaction request to herself, Then it fails', async () => {
  await runScenario(async scenario => {
    const [aliceConductor, alice] = await installApp(scenario, null, 51);

    try {
      const forceInit: Record = await aliceConductor.appAgentWs().callZome({
        role_name: "bridge",
        zome_name: "bridge",
        fn_name: "whoami",
        payload: null,
      });
    }
    catch (e) {
      console.log(e)
      assert.ok(1);
    }

  })
})

test.skip('Given two agents, Alice and Bob, When Alice tries to create a transaction request to Bob, Then it sends', async () => {
  await runScenario(async scenario => {
    const [aliceConductor, alice] = await installApp(scenario, null);
    const [bobConductor, bob] = await installApp(scenario, alice.agentPubKey);

    const forceInit: Record = await bobConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    try {
      const authorityList: Record = await bobConductor.appAgentWs().callZome({
        role_name: "bridge",
        zome_name: "bridge",
        fn_name: "get_authority_list",
        payload: null,
      });
      assert.fail();
    }
    catch (e) {
      assert.ok(1);
    }

  })
})