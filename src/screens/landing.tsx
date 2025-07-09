export default function Landing() {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '60vh',
      }}
    >
      <h1>Bienvenido a YoTeFirmo</h1>
      <span>
        Esta ventana cambiará cuando se deba firmar o cifrar un mensaje
      </span>
    </div>
  );
}
