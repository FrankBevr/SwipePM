import { reactive } from "vue";

interface Store {
  contractAddress: string;
}

export const store: Store = reactive({
  contractAddress: "",
});
