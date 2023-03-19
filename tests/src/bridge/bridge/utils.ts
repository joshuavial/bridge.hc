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
  scenario: Scenario
): Promise<[Conductor, AgentApp]> {
  // Set up the app to be installed
  const appBundle = bridgeApp();

  const aliceConductor = await scenario.addConductor();
  await aliceConductor.attachAppInterface();
  await aliceConductor.connectAppInterface();
  const alicePubKey = await aliceConductor.adminWs().generateAgentPubKey();

  appBundle.manifest.roles.find(
    (r) => r.name === "bridge"
  )!.dna.modifiers = {
    network_seed: "test",
    properties: {
      progenitor_dht_address: serializeHash(alicePubKey),
      progenitor_eth_address: '0xbananas',
      percentage_for_consensus: 51,
    },
  };

   const alice = await aliceConductor.installApp(
    { bundle: appBundle },
    {
      installedAppId: "bridge",
      agentPubKey: alicePubKey,
    }
  );

  await aliceConductor
    .adminWs()
    .enableApp({ installed_app_id: "bridge" });

  await aliceConductor.connectAppAgentInterface("bridge");

  return [aliceConductor, alice];
}
