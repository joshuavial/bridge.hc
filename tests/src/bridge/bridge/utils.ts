import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { decompressSync, unzipSync } from "fflate";

import { AppBundle } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { AgentApp, Conductor, Scenario } from "@holochain/tryorama";

import { Base64 } from "js-base64";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1));
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

export function bridgeApp(): AppBundle {
  const bridgeHapp = path.join(
    __dirname,
    "../../../../workdir/bridge.hc.happ"
  );

  const appBundleBytes = fs.readFileSync(bridgeHapp);

  return decode(decompressSync(new Uint8Array(appBundleBytes))) as any;
}

export async function installApp(
  scenario: Scenario,
  progenitorPubKey: Uint8Array | null,
  percentageForConsensus: Number = 51,
): Promise<[Conductor, AgentApp]> {
  // Set up the app to be installed
  const appBundle = bridgeApp();

  const newConductor = await scenario.addConductor();
  await newConductor.attachAppInterface();
  await newConductor.connectAppInterface();
  const pubKey = await newConductor.adminWs().generateAgentPubKey();

  appBundle.manifest.roles.find(
    (r) => r.name === "bridge"
  )!.dna.modifiers = {
    network_seed: "test",
    properties: {
      progenitor_dht_address: serializeHash(progenitorPubKey || pubKey),
      progenitor_eth_address: '0xbananas',
      percentage_for_consensus: percentageForConsensus,
    },
  };

   const agent = await newConductor.installApp(
    { bundle: appBundle },
    {
      installedAppId: "bridge",
      agentPubKey: pubKey,
    }
  );

  await newConductor
    .adminWs()
    .enableApp({ installed_app_id: "bridge" });

  await newConductor.connectAppAgentInterface("bridge");

  return [newConductor, agent];
}
