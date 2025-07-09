import { useEffect, useRef, useState } from 'react';
import './App.css';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import Landing from './screens/landing';
import { GlobalContext, State } from './context';
import Layout from './screens/layout';

export default function App() {
  const [state, setState] = useState<State>({
    screen: 'landing',
    passphrase: '',
    hash: null,
  });

  const unlisten = useRef<null | UnlistenFn>(null);
  const unlisten2 = useRef<null | UnlistenFn>(null);

  const createListener = async () => {
    unlisten.current = await listen<string>('select-cert', (evt) => {
      const { hash, action } = JSON.parse(evt.payload);
      setState((st) => ({
        ...st,
        screen: 'drop-cert',
        hash,
        action,
      }));
    });
  };

  const createListener2 = async () => {
    unlisten2.current = await listen<string>('verification-info', (evt) => {
      const { cert_info, hash, success } = JSON.parse(evt.payload);
      setState((st) => ({
        ...st,
        screen: 'verification-info',
        certInfo: { ...cert_info, success },
        hash,
        action: 'verification',
      }));
    });
  };

  const submitPassphrase = async (passphraseValue: string) => {
    const isValid = await invoke('update_passphrase', { passphraseValue });

    setState((st) => ({ ...st, screen: 'landing' }));

    return isValid as boolean;
  };

  const submitCertificate = async (certificateValue: Uint8Array) => {
    const certInfo = await invoke('update_certificate', {
      certificateValue,
    });

    if (!certInfo) {
      setState((st) => ({ ...st, screen: 'set-passphrase' }));
    } else {
      setState((st) => ({ ...st, screen: 'landing', hash: null }));
    }
  };

  useEffect(() => {
    invoke('listen_ws_events');
    createListener();
    createListener2();

    return () => {
      unlisten.current && unlisten.current();
      unlisten2.current && unlisten2.current();
    };
  }, []);

  return (
    <main className="container">
      <GlobalContext.Provider
        value={{ submitCertificate, submitPassphrase, ...state }}
      >
        {state.screen === 'landing' && <Landing />}
        {state.screen !== 'landing' && <Layout />}
      </GlobalContext.Provider>
    </main>
  );
}
