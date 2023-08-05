<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { store } from '../store/store';

const winnerTeam = ref("")
const winnerName = ref("")
onMounted(() => {
  if (store.winner === 1) {
    winnerTeam.value = "Manchester";
    winnerName.value = store.participantManchester;
  }
  if (store.winner === 2) {
    winnerTeam.value = "Chelsea"
    winnerName.value = store.participantChelsea;
  }
})

const reset = () => {
  store.contractAddress = "";
  store.allInjected = null;
  store.allAccounts = null;
  store.selectedAccount = null;
  store.injector = null;
  store.isConntected = false;
  store.participantManchester = "";
  store.participantChelsea = "";
  store.winner = 0;
  store.winnerDeclared = false;
}

</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10">
    <p class="leading-relaxed">
      <strong>{{ winnerTeam }}</strong> is the Winner.
      <br />
      <strong>{{ winnerName }}</strong> needs some new piggy banks, they are getting full 🐖🪙
      <br />
      🎉 Whuhuu! 🎉
      <br />
      {{}}
      <br />
    <div>This Demo was made via:
      <ul class="text-left">
        <li>UI/UX creation via <a href="#">Figma</a></li>
        <li>Smart Contract creation via Rust with <a href="#">ink!</a></li>
        <li>Frontend creation via Typescript with <a href="#">polkadotJS</a></li>
      </ul>
    </div>
    Thanks for this fun Hack.
    <br />
    Shoutout to the <a href="#">organizers</a>.
    <br />
    <br />
    <br />
    <div>Next steps are the following
      <ul class="text-left">
        <li>Get contract in shape and make a ZTG Call via ink</li>
        <li>Get frontend in shape to work with newly Contract</li>
        <li>Be happy 🌤️</li>
      </ul>
    </div>
    <br />
    <div>So enough talking, <strong>WANNA RESTART</strong>, mister Admin.</div>
    </p>
    <button @click="reset"
      class="bg-#CCCCCC text-2xl p-5 b-rd-3 hover:bg-#ffffff outline outline-1 font-light border-none">
      RESTART
    </button>
  </div>
</template>
