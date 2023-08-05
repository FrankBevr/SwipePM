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

const app = createApp({
  components: {
    Header,
    Duel,
    Connect,
    PlaygroundCallButtons,
  },
  template: `
    <Header/>
    <Duel/>
    <Connect/>
    <PlaygroundCallButtons/>
  `,
});

app.mount("#app");
