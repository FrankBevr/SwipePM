<script setup lang="ts">
import GetGameButton from "./GetGameButton.vue";
import SetWinnerButton from "./SetWinnerButton.vue";
import SetParticipantChelseaButton from "./SetParticipantChelseaButton.vue";
import SetParticpantManchesterButton from "./SetParticipantManchesterButton.vue";
import SetStoreContractAddress from "./SetStoreContractAddress.vue";
import SetAccount from "./SetAccount.vue";
</script>

<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <GetGameButton></GetGameButton>
  <SetWinnerButton></SetWinnerButton>
  <SetParticipantChelseaButton></SetParticipantChelseaButton>
  <SetParticpantManchesterButton></SetParticpantManchesterButton>
  <SetAccount></SetAccount>
  <SetStoreContractAddress></SetStoreContractAddress>
</template>
