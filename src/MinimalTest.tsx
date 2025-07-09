function MinimalTest() {
  return (
    <div
      style={{
        padding: '20px',
        fontFamily: 'Arial, sans-serif',
        backgroundColor: '#f0f0f0',
        minHeight: '100vh',
      }}
    >
      <h1 style={{ color: 'blue' }}>✅ React is Working!</h1>
      <p>If you can see this, React is rendering properly.</p>
      <button
        onClick={() => alert('Button works!')}
        style={{
          padding: '10px 20px',
          fontSize: '16px',
          backgroundColor: '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '4px',
          cursor: 'pointer',
        }}
      >
        Test Button
      </button>
    </div>
  );
}

export default MinimalTest;
