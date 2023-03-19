import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createApproval, sampleApproval } from './common.js';

import {installApp} from './utils.js';

test('add new authorities to list', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    //
    const [aliceConductor, alice] = await installApp(scenario, null);
    const [bobConductor, bob] = await installApp(scenario, alice.agentPubKey);
    const [samConductor, sam] = await installApp(scenario, alice.agentPubKey);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const forceInit: any = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "whoami",
      payload: null,
    });

    const original_auth_list: any = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "get_authority_list",
      payload: null,
    });

    const authList2 = {
      percentage_for_consensus: 51,
      authorities: [[alice.agentPubKey, 'bananas'], [bob.agentPubKey, 'oranges'], [sam.agentPubKey, 'grapes']]
    }
    // Alice creates a Approval
    const newApproval = {
      proposed_entry: { AuthorityList: authList2 },
      approved_by: [],
    }
    const create_action_hash: String = await createApproval(alice.cells[0], newApproval);
    assert.ok(create_action_hash);

    const update_auth_list_action_hash: any = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "update_authority_list",
      payload: authList2,
    });
    assert.fail('should not be able to update auth list without signatures')
  });
});

test.skip('create and read Approval', async () => {
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

    const sample = await sampleApproval(alice.cells[0]);

    // Alice creates a Approval
    const record: Record = await createApproval(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

  // Bob gets the created Approval
  const createReadOutput: Record = await bob.cells[0].callZome({
    zome_name: "bridge",
    fn_name: "get_approval",
    payload: record.signed_action.hashed.hash,
  });
  assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test.skip('create and update Approval', async () => {
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

    // Alice creates a Approval
    const record: Record = await createApproval(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the Approval
    let contentUpdate: any = await sampleApproval(alice.cells[0]);
    let updateInput = {
      original_approval_hash: originalActionHash,
      previous_approval_hash: originalActionHash,
      updated_approval: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "update_approval",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);

  // Bob gets the updated Approval
  const readUpdatedOutput0: Record = await bob.cells[0].callZome({
    zome_name: "bridge",
    fn_name: "get_approval",
    payload: updatedRecord.signed_action.hashed.hash,
  });
  assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

  // Alice updates the Approval again
  contentUpdate = await sampleApproval(alice.cells[0]);
  updateInput = { 
    original_approval_hash: originalActionHash,
    previous_approval_hash: updatedRecord.signed_action.hashed.hash,
    updated_approval: contentUpdate,
  };

  updatedRecord = await alice.cells[0].callZome({
    zome_name: "bridge",
    fn_name: "update_approval",
    payload: updateInput,
  });
  assert.ok(updatedRecord);

  // Wait for the updated entry to be propagated to the other node.
  await pause(1200);

// Bob gets the updated Approval
const readUpdatedOutput1: Record = await bob.cells[0].callZome({
  zome_name: "bridge",
  fn_name: "get_approval",
  payload: updatedRecord.signed_action.hashed.hash,
});
assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test.skip('create and delete Approval', async () => {
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

    // Alice creates a Approval
    const record: Record = await createApproval(alice.cells[0]);
    assert.ok(record);

    // Alice deletes the Approval
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "bridge",
      fn_name: "delete_approval",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);

  // Bob tries to get the deleted Approval
  const readDeletedOutput = await bob.cells[0].callZome({
    zome_name: "bridge",
    fn_name: "get_approval",
    payload: record.signed_action.hashed.hash,
  });
  assert.notOk(readDeletedOutput);
  });
});
