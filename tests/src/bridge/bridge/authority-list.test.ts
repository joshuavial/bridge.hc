import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createAuthorityList, sampleAuthorityList } from './common.js';

import {installApp} from './utils.js';

test('AuthorityList generated from properties and progenitor', async () => {
  await runScenario(async scenario => {

    const [aliceConductor, alice] = await installApp(scenario);

    const record: Record = await aliceConductor.appAgentWs().callZome({
      role_name: "bridge",
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: null,
    });

    //const aliceBridgeCell = 

    // Alice creates a AuthorityList
    //const record: Record = await createAuthorityList(alice.cells[0]);
    assert.ok(record);
  });
});

test.skip('create and read AuthorityList', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/bridge.hc.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sample = await sampleAuthorityList(alice.cells[0]);

    // Alice creates a AuthorityList
    const record: Record = await createAuthorityList(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created AuthorityList
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test.skip('create and update AuthorityList', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/bridge.hc.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a AuthorityList
    const record: Record = await createAuthorityList(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the AuthorityList
    let contentUpdate: any = await sampleAuthorityList(alice.cells[0]);
    let updateInput = {
      previous_authority_list_hash: originalActionHash,
      updated_authority_list: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "update_authority_list",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated AuthorityList
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the AuthorityList again
    contentUpdate = await sampleAuthorityList(alice.cells[0]);
    updateInput = { 
      previous_authority_list_hash: updatedRecord.signed_action.hashed.hash,
      updated_authority_list: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "update_authority_list",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated AuthorityList
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

