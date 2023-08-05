<script setup lang="ts">
import { store } from "../store/store";
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10">
    <h1>History</h1>
  </div>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10"
    v-if="store.participantChelsea === '' && store.participantManchester === ''">
    <p class="leading-relaxed"><strong>Noone</strong> voted yet.</p>
  </div>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10"
    v-if="store.participantChelsea !== ''">
    <p class="leading-relaxed">
      <strong>{{ store.participantChelsea }}</strong> voted for Chelsea.
      <br />
      🎉 {{ store.participantChelsea }} is an amazing voter 🎉
    </p>
  </div>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10"
    v-if="store.participantManchester !== ''">
    <p class="leading-relaxed">
      <strong>{{ store.participantManchester }}</strong> voted for Manchester.
      <br />
      🎉 {{ store.participantManchester }} is an amazing voter 🎉
    </p>
  </div>
  <div class="h-20px w-20px"></div>
</template>
