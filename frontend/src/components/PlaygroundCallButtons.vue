<script setup lang="ts">
import GetGameButton from "./GetGameButton.vue";
import SetWinnerButton from "./SetWinnerButton.vue";
import SetParticipantChelseaButton from "./SetParticipantChelseaButton.vue";
</script>

<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <GetGameButton></GetGameButton>
  <SetWinnerButton></SetWinnerButton>
  <SetParticipantChelseaButton></SetParticipantChelseaButton>
  <div class="flex flex-justify-center p-10">
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      setParticpantManchester
    </button>
  </div>
  <h1 class="flex flex-justify-center">Accounts</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.accounts }} -->
    </pre>
  </div>
  <h1 class="flex flex-justify-center">selectedAccount</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.selectedAccount }} -->
    </pre>
  </div>
  <h1 class="flex flex-justify-center">Metadata</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.metadata }} -->
    </pre>
  </div>
</template>
