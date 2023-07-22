import "./style.css";
import { createApp } from "vue";
import Header from "./header.ts";
import Duel from "./duel.ts";

const app = createApp({
  components: {
    Header,
    Duel,
  },
  template: `
    <Header/>
    <Duel/>
  `,
});

app.mount("#app");
