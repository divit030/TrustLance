
import { useState } from "react";
import BN from "bn.js";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import { useServiceHubProgram } from "../config/anchor";

export default function RegisterProvider() {
  const { publicKey } = useWallet();
  const program = useServiceHubProgram();

  const [name, setName] = useState("");
  const [service, setService] = useState("");
  const [price, setPrice] = useState("");
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);

  async function registerProvider(event) {
    event.preventDefault();

    if (!publicKey || !program) {
      setStatus("Connect your Phantom wallet first.");
      return;
    }

    if (!name.trim() || !service.trim()) {
      setStatus("Please fill in all fields.");
      return;
    }

    const priceInSol = Number(price);

    if (!Number.isFinite(priceInSol) || priceInSol <= 0) {
      setStatus("Enter a valid price greater than zero.");
      return;
    }

    const priceInLamports = Math.round(priceInSol * 1_000_000_000);

    if (!Number.isSafeInteger(priceInLamports) || priceInLamports <= 0) {
      setStatus("The price is too large or too precise.");
      return;
    }

    try {
      setLoading(true);
      setStatus("Registering provider...");

      const [providerPda] = PublicKey.findProgramAddressSync(
  [new TextEncoder().encode("provider"), publicKey.toBuffer()],
  program.programId
);

      const signature = await program.methods
        .registerProvider(
          name.trim(),
          service.trim(),
          new BN(priceInLamports)
        )
        .accounts({
          authority: publicKey,
          provider: providerPda,
          systemProgram: SystemProgram.programId
        })
        .rpc();

      setStatus(`Provider registered successfully! Transaction: ${signature}`);
    } catch (error) {
      console.error(error);
      setStatus(error.message || "Provider registration failed.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div>
      <h2>Register as Provider</h2>

      <form onSubmit={registerProvider}>
        <input
          value={name}
          onChange={(event) => setName(event.target.value)}
          placeholder="Provider name"
          maxLength={50}
          required
        />

        <input
          value={service}
          onChange={(event) => setService(event.target.value)}
          placeholder="Service offered"
          maxLength={50}
          required
        />

        <input
          type="number"
          min="0.000000001"
          step="0.000000001"
          value={price}
          onChange={(event) => setPrice(event.target.value)}
          placeholder="Price in SOL"
          required
        />

        <button type="submit" disabled={loading || !program}>
          {loading ? "Registering..." : "Register Provider"}
        </button>
      </form>

      <p>{status}</p>
    </div>
  );
}
