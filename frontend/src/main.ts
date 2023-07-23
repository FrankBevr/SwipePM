import "./style.css";
import "virtual:uno.css";
import { createApp } from "vue";
import Header from "./components/Header.vue";
import Duel from "./components/Duel.vue";
import Connect from "./components/Connect.vue";
import PlaygroundCallButtons from "./components/PlaygroundCallButtons.vue";

const app = createApp({
  components: {
    Header,
    Duel,
    Connect,
  },
  template: `
    <Header/>
    <Duel/>
    <!-- <Connect/> -->
    <!-- <PlaygroundCallButtons/> -->
  `,
});

app.mount("#app");
