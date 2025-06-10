import { useEffect, useRef, useState } from 'react';
import './App.css';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import DropCertificate from './screens/drop-certificate';
import SetPassphrase from './screens/set-passphrase';
import Landing from './screens/landing';
import { GlobalContext } from './context';

interface State {
  screen: 'landing' | 'drop-cert' | 'set-passphrase';
  passphrase: string;
  hash: null | string;
}

export default function App() {
  const [state, setState] = useState<State>({
    screen: 'landing',
    passphrase: '',
    hash: null,
  });
  const unlisten = useRef<null | UnlistenFn>(null);

  const createListener = async () => {
    unlisten.current = await listen<string>('select-cert', (evt) => {
      setState((st) => ({ ...st, screen: 'drop-cert', hash: evt.payload }));
    });
  };

  const submitPassphrase = async (passphraseValue: string) => {
    const _certInfo = await invoke('update_passphrase', { passphraseValue });

    setState((st) => ({ ...st, screen: 'landing' }));
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
    return () => {
      unlisten.current && unlisten.current();
    };
  }, []);

  return (
    <main className="container">
      <GlobalContext.Provider
        value={{ submitCertificate, submitPassphrase, ...state }}
      >
        {state.screen === 'drop-cert' && <DropCertificate />}
        {state.screen === 'landing' && <Landing />}
        {state.screen === 'set-passphrase' && <SetPassphrase />}
      </GlobalContext.Provider>
    </main>
  );
}
