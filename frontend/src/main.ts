import "./style.css";
import "virtual:uno.css";
import { createApp } from "vue";
import Header from "./components/Header.vue";
import Duel from "./components/Duel.vue";
import Connect from "./components/Connect.vue";

const app = createApp({
  components: {
    Header,
    Duel,
    Connect,
  },
  template: `
    <Connect/>
    <Header/>
    <Duel/>
  `,
});

app.mount("#app");
