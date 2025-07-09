import { createContext } from 'react';

export type Action = 'sign' | 'decrypt' | 'verification';

export interface CertInfo {
  iat: string;
  exp: string;
  issuer: string;
  subj: string;
  success: boolean;
}

export interface State {
  screen: 'landing' | 'drop-cert' | 'set-passphrase' | 'verification-info';
  passphrase: string;
  hash: null | Buffer;
  action?: Action;
  certInfo?: CertInfo;
}

export interface GlobalContext extends State {
  submitCertificate: (cert: Uint8Array) => Promise<void>;
  submitPassphrase: (passphrase: string) => Promise<boolean>;
}

export const GlobalContext = createContext<GlobalContext | null>(null);
