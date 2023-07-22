import "./style.css";
import { createApp, ref } from "vue";

const app = createApp({
  setup() {
    const message = ref("SwipePM");
    return { message };
  },
});

app.mount("#app");
