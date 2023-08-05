<script setup lang="ts">
import GetGameButton from "./playground/GetGameButton.vue";

import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";
import { store } from "../store/store";
import { watchEffect } from "vue";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

async function set_winner(winner: number) {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });
  console.log(store.selectedAccount)

  const contract = new ContractPromise(api, metadata, store.contractAddress);
  const storageDepositLimit = null;
  const { gasRequired } = await contract.query["footballMatch::setWinner"](
    store.selectedAccount!.address,
    {
      gasLimit: api?.registry.createType("WeightV2", {
        refTime: MAX_CALL_WEIGHT,
        proofSize: PROOFSIZE,
      }) as WeightV2,
      storageDepositLimit,
    },
    winner,
  );

  const gasLimit = api?.registry.createType(
    "WeightV2",
    gasRequired,
  ) as WeightV2;

  await contract.tx["footballMatch::setWinner"](
    {
      gasLimit,
      storageDepositLimit,
    },
    winner,
  ).signAndSend(store.selectedAccount?.address!, { signer: store.injector!.signer }, (res) => {
    if (res.isInBlock) {
      call_getGame()
    }
  });
};

const call_getGame = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const address = store.contractAddress;
  const contract = new ContractPromise(api, metadata, address);

  const storageDepositLimit = null;

  const { result, output } = await contract.query["footballMatch::getGame"](
    alice.address,
    {
      gasLimit: api?.registry.createType("WeightV2", {
        refTime: MAX_CALL_WEIGHT,
        proofSize: PROOFSIZE,
      }) as WeightV2,
      storageDepositLimit,
    },
  );

  if (result.isOk) {
    const outy = output?.toJSON() as { ok: { ok: number } };
    if (outy !== undefined) {
      store.winner = outy["ok"]["ok"];
      store.winnerDeclared = true;
    }
  } else {
    console.error("Error", result.asErr);
  }
};
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10">
    <p class="leading-relaxed">
      <strong>Set Winner</strong>, Mr. Admin.
      <br />
      Check the the result and send it to the contract.
      <br />
    <div>For Demo Sake:
      <ul class="text-left">
        <li>No winner defined yet, 0 is declared.</li>
        <li>Send 1 to declare Manchester as the Winner</li>
        <li>Send 2 to declare Chelsea as the Winner</li>
      </ul>
    </div>
    <div><strong>Who won?</strong>
      <br />The power relies in your hands
    </div>
    </p>
    <button @click="set_winner(1)"
      class="bg-#CCCCCC text-2xl p-5 b-rd-3 hover:bg-#ffffff outline outline-1 font-light border-none">
      Manchester won
    </button>
    <button @click="set_winner(2)"
      class="bg-#CCCCCC text-2xl p-5 b-rd-3 hover:bg-#ffffff outline outline-1 font-light border-none mt-10">
      Chelsea won
    </button>
  </div>
  <details class="p-5 cursor-pointer">
    <summary class="flex flex-justify-center">&#9205; Wann check the result?</summary>
    <GetGameButton />
  </details>
</template>

