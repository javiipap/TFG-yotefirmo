import { useContext, useEffect, useRef, useState } from 'react';
import { GlobalContext } from '../context';

export default function SetPassphrase() {
  const [passphrase, setPassphrase] = useState('');
  const { submitPassphrase } = useContext(GlobalContext) as GlobalContext;
  const inputRef = useRef<HTMLInputElement>(null);

  const onSubmit = async () => {
    const isValid = await submitPassphrase(passphrase);

    if (!isValid) {
      alert('Invalid password');
      setPassphrase('');
    }
  };

  useEffect(() => {
    inputRef.current?.focus();
    // @ts-ignore Empty dependency array
  }, []);

  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'flex-start',
        flexDirection: 'column',
      }}
    >
      <span>Passphrase</span>
      <div style={{ marginTop: 8 }}>
        <input
          type="password"
          name="passphrase"
          value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          id=""
          ref={inputRef}
          style={{ marginRight: 8 }}
        />
        <button onClick={onSubmit}>Enviar</button>
      </div>
    </div>
  );
}
