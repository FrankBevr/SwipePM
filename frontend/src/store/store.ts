import { reactive } from "vue";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ApiPromise } from "@polkadot/api";

interface Store {
  accounts: InjectedAccountWithMeta[] | null;
  selectedAccount: InjectedAccountWithMeta | null;
  api: ApiPromise | null;
}

export const store: Store = reactive({
  accounts: null,
  selectedAccount: null,
  api: null,
});
