import { createContext } from 'react';

export interface GlobalContext {
  submitCertificate: (cert: Uint8Array) => Promise<void>;
  submitPassphrase: (passphrase: string) => void;
  passphrase: string;
  hash: null | string;
}

export const GlobalContext = createContext<GlobalContext | null>(null);
