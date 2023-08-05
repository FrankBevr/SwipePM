import "./style.css";
import "virtual:uno.css";
import { createApp } from "vue";
//@ts-ignore
import Header from "./components/Header.vue";
//@ts-ignore
import Duel from "./components/Duel.vue";
//@ts-ignore
import Connect from "./components/Connect.vue";
//@ts-ignore
import PlaygroundCallButtons from "./components/playground/PlaygroundCallButtons.vue";
import { store } from "./store/store.ts";

const app = createApp({
  components: {
    Header,
    Duel,
    Connect,
    PlaygroundCallButtons,
  },
  setup() {
    return {
      store,
    };
  },
  template: `
    <Header/>
    <Duel v-if="store.isConntected"/>
    <Connect v-else="store.isConntected"/>
    <details class="p-10 cursor-pointer">
      <summary class="flex flex-justify-center">&#9205; Wann see a Playground?</summary>
      <div class="flex flex-justify-center">Scroll down</div>
      <PlaygroundCallButtons/>
    </details>
  `,
});

app.mount("#app");
