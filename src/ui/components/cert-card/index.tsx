import { Link } from 'react-router-dom/dist';
// const image = require('./cert-icon.webp');

export interface Props {
  name: string;
  issuer: string;
  extensions: string;
  iat: Date;
  exp: Date;
  id: string;
}

export default function CertCard({
  name,
  issuer,
  extensions,
  iat,
  exp,
  id,
}: Props) {
  const showCert = () => {};
  return (
    <div className="p-4 border-b flex items-center space-x-4">
      <div className="h-[5em] w-auto">
        <img
          // src={image}
          alt=""
          className="h-full w-auto aspect-square !max-w-lg"
        />
      </div>
      <div className="">
        <h2 className="font-bold text-xl">{name}</h2>
        <p className="text-sm">
          Emisor: {issuer}. Uso {extensions}
        </p>
        <p className="text-sm">
          Válido desde {iat.toLocaleDateString()} hasta{' '}
          {exp.toLocaleDateString()}
        </p>
        <a className="text-sm underline">Propiedades del certificado</a>
      </div>
    </div>
  );
}
