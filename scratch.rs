

// -AuthorityList

//     - list of authorities (holo_pubkey, eth_pubkey)

//     - initial list provided by the properties of the dht

//     - validations
//         at least 50% sign an update to the list

struct AuthorityList {
    authorities: Vec<(AgentPubKey, String)>
    percentage_for_consensus: u32,
}

//  need validation for:
//  - percentage for consensus must be at least 50%
//  - authorities list must be of a minimum length
