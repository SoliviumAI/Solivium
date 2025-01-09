import React, { useState } from 'react';

function App() {
  const [wallet, setWallet] = useState("");
  const [balance, setBalance] = useState(null);

  const fetchBalance = async () => {
    const response = await fetch(`http://localhost:8000/balance/${wallet}`);
    const data = await response.json();
    setBalance(data.balance);
  };

  return (
    <div>
      <h1>Solivium: Decentralized Personal Assistant</h1>
      <input
        type="text"
        placeholder="Enter Wallet Address"
        value={wallet}
        onChange={(e) => setWallet(e.target.value)}
      />
      <button onClick={fetchBalance}>Get Balance</button>
      {balance !== null && <p>Wallet Balance: {balance} SOL</p>}
    </div>
  );
}

export default App;
