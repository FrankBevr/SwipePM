import "./style.css";

const app = Vue.createApp({
  setup() {
    const message = Vue.ref("SwipePM");
    return { message };
  },
});

app.mount("#app");
