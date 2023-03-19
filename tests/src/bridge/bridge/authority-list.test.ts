import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createAuthorityList, sampleAuthorityList } from './common.js';

import {installApp} from './utils.js';


test('AuthorityList generated from properties by progenitor', async () => {
  await runScenario(async scenario => {

    const [aliceConductor, alice] = await installApp(scenario, null);

    const forceInit: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    const authorityList: any = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: null,
    });

    //const aliceBridgeCell = 

    // Alice creates a AuthorityList
    //const record: Record = await createAuthorityList(alice.cells[0]);
    assert.equal(authorityList.percentage_for_consensus, 51);
  });
});

test('cannot have authority list with consensus percentage <= 50', async () => {
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

test('cannot see authority list if progenitor has not initialised', async () => {
  await runScenario(async scenario => {
    const [aliceConductor, alice] = await installApp(scenario, null);
    const [bobConductor, bob] = await installApp(scenario, alice.agentPubKey);

    await scenario.shareAllAgents();

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
