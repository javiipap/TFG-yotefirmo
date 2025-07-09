import { useContext } from 'react';
import { GlobalContext } from '../context';
import DropCertificate from './drop-certificate';
import SetPassphrase from './set-passphrase';
import VerificationInfo from './verification-info';

export default function Layout() {
  const { action, hash, screen } = useContext(GlobalContext) as GlobalContext;

  const actionText = action === 'decrypt' ? 'descifrado' : 'firma';

  return (
    <div style={{ display: 'flex', flexDirection: 'column', padding: 80 }}>
      <h1>
        {action === 'verification'
          ? 'Verificación de firma'
          : `Solicitud de ${actionText}`}
      </h1>
      <div
        style={{
          textAlign: 'left',
          background: '#1c1c1c',
          padding: '8px 16px',
          borderRadius: 8,
        }}
      >
        <span style={{ fontWeight: 'bold' }}>Payload</span>
        <hr style={{ border: 'none', height: 1, background: 'white' }} />
        <div
          style={{
            resize: 'none',
            color: 'white',
            textAlign: 'left',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
          }}
        >
          {hash}
        </div>
      </div>
      <div style={{ marginTop: 16 }}>
        {screen === 'drop-cert' && <DropCertificate />}
        {screen === 'set-passphrase' && <SetPassphrase />}
        {screen === 'verification-info' && <VerificationInfo />}
      </div>
    </div>
  );
}
