import { ChangeEvent, useContext, useState } from 'react';
import { GlobalContext } from '../context';

export default function DropCertificate() {
  const [certificate, setCertificate] = useState<File>();

  const { submitCertificate } = useContext(GlobalContext) as GlobalContext;

  const onChange = async (e: ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;

    if (!files || files.length < 1) {
      return;
    }

    setCertificate(files[0]);
  };

  return (
    <div className="">
      <input type="file" name="certificate" onChange={onChange} id="" />
      <button
        onClick={async () =>
          !!certificate &&
          submitCertificate(new Uint8Array(await certificate.arrayBuffer()))
        }
      >
        Seleccionar
      </button>
    </div>
  );
}
