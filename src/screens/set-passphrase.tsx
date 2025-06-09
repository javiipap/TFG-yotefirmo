import { useContext, useState } from 'react';
import { GlobalContext } from '../context';

export default function SetPassphrase() {
  const [passphrase, setPassphrase] = useState('');
  const { submitPassphrase } = useContext(GlobalContext) as GlobalContext;

  return (
    <div className="">
      <input
        type="password"
        name="passphrase"
        value={passphrase}
        onChange={(e) => setPassphrase(e.target.value)}
        id=""
      />
      <button onClick={() => submitPassphrase(passphrase)}>Enviar</button>
    </div>
  );
}
