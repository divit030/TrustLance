
import { useEffect, useState } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

export default function WalletInfo() {
  const { connection } = useConnection();
  const { publicKey, connected } = useWallet();

  const [balance, setBalance] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let active = true;

    async function fetchBalance() {
      if (!publicKey || !connected) {
        setBalance(null);
        return;
      }

      try {
        setLoading(true);

        const lamports = await connection.getBalance(publicKey);

        if (active) {
          setBalance(lamports / LAMPORTS_PER_SOL);
        }
      } catch (error) {
        console.error("Unable to fetch balance:", error);
      } finally {
        if (active) {
          setLoading(false);
        }
      }
    }

    fetchBalance();

    return () => {
      active = false;
    };
  }, [connection, publicKey, connected]);

  if (!connected || !publicKey) {
    return <p>Connect your wallet to view your account details.</p>;
  }

  return (
    <div>
      <h2>Wallet Information</h2>

      <p>
        <strong>Wallet Address:</strong>
        {publicKey.toBase58()}
      </p>

      <p>
        <strong>SOL Balance:</strong>
        {loading ? "Loading..." : balance === null ? "Unavailable" : `${balance} SOL`}
      </p>
    </div>
  );
}
