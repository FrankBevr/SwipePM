import { reactive } from "vue";
import type {
  InjectedExtension,
  InjectedAccountWithMeta,
} from "@polkadot/extension-inject/types";

interface Store {
  contractAddress: string;
  allInjected: null | InjectedExtension[];
  allAccounts: null | InjectedAccountWithMeta[];
  selectedAccount: null | InjectedAccountWithMeta;
  injector: null | InjectedExtension;
  isConntected: boolean;
  participantManchester: string;
  participantChelsea: string;
  winnerDeclared: boolean;
  winner: number;
}

export let store: Store = reactive({
  contractAddress: "",
  allInjected: null,
  allAccounts: null,
  selectedAccount: null,
  injector: null,
  isConntected: false,
  participantManchester: "",
  participantChelsea: "",
  winnerDeclared: false,
  winner: 0,
});
