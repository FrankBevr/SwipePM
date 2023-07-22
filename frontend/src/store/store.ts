import { reactive } from "vue";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ApiPromise } from "@polkadot/api";
import * as metadata from "./football_match.json";

interface Store {
  accounts: InjectedAccountWithMeta[] | null;
  selectedAccount: InjectedAccountWithMeta | null;
  api: ApiPromise | null;
  metadata: any;
}

export const store: Store = reactive({
  accounts: null,
  selectedAccount: null,
  api: null,
  metadata: metadata,
});
