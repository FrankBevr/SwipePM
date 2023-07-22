import { reactive } from "vue";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

interface Store {
  accounts: InjectedAccountWithMeta[] | null;
  selectedAccount: InjectedAccountWithMeta | null;
}

export const store: Store = reactive({
  accounts: null,
  selectedAccount: null,
});
