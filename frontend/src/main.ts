import "./style.css";
import { createApp } from "vue";
import Header from "./components/Header.vue";
import Duel from "./components/Duel.vue";

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
