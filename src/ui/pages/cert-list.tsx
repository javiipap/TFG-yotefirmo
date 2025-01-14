import CertList from '../components/cert-list';

export default function CertSelect() {
  return (
    <div className="p-8">
      <div className="">
        <h1 className="text-2xl font-bold">Selecciona el certificado</h1>
        <div className="my-6">
          <CertList />
        </div>
      </div>
      <div className="flex justify-end">
        <button className="bg-black text-white px-4 py-2 rounded">
          Continuar
        </button>
      </div>
    </div>
  );
}
