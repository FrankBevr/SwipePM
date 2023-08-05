<script setup lang="ts">
import {
  web3Accounts,
  web3Enable,
  web3FromAddress,
} from "@polkadot/extension-dapp";
import { Ref, ref } from "vue";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { store } from "../../store/store";

const successTransaction: Ref<boolean> = ref(false);

async function get_accounts() {
  store.allInjected = await web3Enable("my cool dapp");
  store.allAccounts = await web3Accounts();
}
async function select_account(index: number) {
  store.allInjected = await web3Enable("my cool dapp");
  store.allAccounts = await web3Accounts();
  store.selectedAccount = store.allAccounts[index];
}
async function send_transaction() {
  store.allInjected = await web3Enable("my cool dapp");
  store.allAccounts = await web3Accounts();
  store.selectedAccount = store.allAccounts[3];

  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const SENDER = store.selectedAccount.address;
  store.injector = await web3FromAddress(SENDER);
  api.tx.balances
    .transfer("5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ", 123456)
    .signAndSend(SENDER, { signer: store.injector.signer }, (status) => {
      successTransaction.value = status.isInBlock;
    });
}
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline outline-1 items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none">
    <p class="leading-relaxed">
      <strong>get_accounts</strong> gets all the accounts which are injected
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="get_accounts">
      get_accounts
    </button>

    <p class="leading-relaxed">
      <strong>select_account</strong> gets all the accounts and sets the account
      based on the chosen index.
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="select_account(3)">
      select_account(3)
    </button>
    <p class="leading-relaxed">
      <strong>send_transaction()</strong> sends transaction to address <br />
      gets all the accounts which are injected. <br />
      selects third account which is injected. <br />
      sets up api <br />
      send Transaction to 5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="send_transaction">
      send_transaction
    </button>
    <p v-if="successTransaction">Its was successfull</p>
    <p class="leading-relaxed">show all Accounts</p>
    <pre class="font-light text-10px text-left w-full" v-if="store.allAccounts">
      {{ store.allAccounts }}
    </pre>
    <pre class="font-light text-10px text-left w-full" v-else>
Press get_accounts Buttons</pre>
    <p class="leading-relaxed">show selected Accounts</p>
    <pre class="font-light text-10px text-left w-full" v-if="store.selectedAccount">
     {{ store.selectedAccount }}
    </pre>
    <pre class="font-light text-10px text-left w-full" v-else>
Press get_accounts() and then select_accounts(3) Buttons</pre>
    <p class="leading-relaxed">shows metadata of an selected Accounts</p>
    <pre class="font-light text-10px text-left w-full" v-if="store.selectedAccount">
      {{ store.selectedAccount.meta }}
    </pre>
    <pre class="font-light text-10px text-left w-full" v-else>
Press get_accounts() and then select_accounts(3) Buttons</pre>
  </div>
</template>
