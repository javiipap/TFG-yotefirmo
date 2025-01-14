import CertificateCard, { Props } from '../cert-card';

export default function CertList() {
  const mockCertificate = {
    name: 'Certificado 1',
    issuer: 'AC representación',
    extensions: 'Firma y autenticación',
    iat: new Date(),
    exp: new Date(),
    id: '8831f-13241fqasdf-1324-dfa',
  } as Props;

  return (
    <div className="divide-y divide-gray-300 h-[400px] overflow-y-auto">
      {[...new Array(5).keys()]
        .map((i) => i + 1)
        .map((i) => (
          <CertificateCard key={`cert-${i}`} {...mockCertificate} />
        ))}
    </div>
  );
}
