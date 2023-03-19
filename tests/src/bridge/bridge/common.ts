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


export async function sampleApproval(cell: CallableCell, partialApproval = {}) {
    return {
        ...{
	  authority_list: (await fakeEntryHash()),
	  timestamp: 1674053334548000,
	  approved_by: [(await fakeAgentPubKey())],
        },
        ...partialApproval
    };
}

export async function createApproval(cell: CallableCell, approval = undefined): Promise<String> {
    return cell.callZome({
      zome_name: "bridge",
      fn_name: "create_approval",
      payload: approval || await sampleApproval(cell),
    });
}

