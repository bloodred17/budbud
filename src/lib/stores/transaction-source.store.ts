import {writable} from "svelte/store";
import {invoke} from "@tauri-apps/api/tauri";
import type {ResponseData} from "$lib/types/communication.interface";

export interface TransactionSource {
  id:               TransactionSourceID;
  name:             string;
  transaction_type: string;
}

export interface TransactionSourceID {
  tb: string;
  id: IDID;
}

export interface IDID {
  String?: string;
}

export const transactionSourceStore = writable<TransactionSource[]>([]);

export function fetchTransactionSources(callback?: () => void) {
  invoke<string>('list_transaction_sources')
    .then((responseStr: string) => {
      const response: ResponseData<TransactionSource[]> = JSON.parse(responseStr);
      transactionSourceStore.update(() => response?.data);
      if (callback) {
        callback();
      }
    })
}