import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleAuthorityList(cell: CallableCell, partialAuthorityList = {}) {
    return {
        ...{
	  percentage_for_consensus: 10,
	  authorities: ["Lorem ipsum dolor sit amet, consectetur adipiscing elit."],
        },
        ...partialAuthorityList
    };
}

export async function createAuthorityList(cell: CallableCell, authorityList = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "bridge",
      fn_name: "create_authority_list",
      payload: authorityList || await sampleAuthorityList(cell),
    });
}

