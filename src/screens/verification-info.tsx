import { useContext } from 'react';
import { CertInfo, GlobalContext } from '../context';

export default function VerificationInfo() {
  const context = useContext(GlobalContext) as GlobalContext;

  const certInfo = context.certInfo as CertInfo;

  return <>{JSON.stringify(certInfo)}</>;
}
