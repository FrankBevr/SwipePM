import "./style.css";
import { createApp } from "vue";
import Header from "./components/header.ts";
import Duel from "./components/duel.ts";

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
