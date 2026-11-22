
import { useEffect, useState } from "react";
import BN from "bn.js";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import { useServiceHubProgram } from "../config/anchor";

export default function Marketplace() {
  const { publicKey } = useWallet();
  const program = useServiceHubProgram();

  const [providers, setProviders] = useState([]);
  const [loading, setLoading] = useState(false);
  const [bookingProvider, setBookingProvider] = useState(null);
  const [status, setStatus] = useState("");

  async function loadProviders() {
    if (!program) return;

    try {
      setLoading(true);
      setStatus("Loading providers...");

      const accounts = await program.account.provider.all();

      setProviders(
        accounts.map((item) => ({
          publicKey: item.publicKey,
          ...item.account
        }))
      );

      setStatus("");
    } catch (error) {
      console.error(error);
      setStatus("Unable to load providers. Check the browser console.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadProviders();
  }, [program]);

  async function createBooking(provider) {
    if (!publicKey || !program) {
      setStatus("Connect your Phantom wallet first.");
      return;
    }

    try {
      setBookingProvider(provider.publicKey.toBase58());
      setStatus("Creating booking and depositing SOL into escrow...");

      const [customerPda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode("customer"), publicKey.toBuffer()],
        program.programId
      );

      await program.account.customer.fetch(customerPda);

      const bookingId = new BN(Date.now().toString());
      const bookingIdBytes = new Uint8Array(bookingId.toArray("le", 8));

      const [bookingPda] = PublicKey.findProgramAddressSync(
        [
          new TextEncoder().encode("booking"),
          publicKey.toBuffer(),
          bookingIdBytes
        ],
        program.programId
      );

      const [escrowPda] = PublicKey.findProgramAddressSync(
        [
          new TextEncoder().encode("escrow"),
          bookingPda.toBuffer()
        ],
        program.programId
      );

      const signature = await program.methods
        .createBooking(bookingId)
        .accounts({
          customerAuthority: publicKey,
          customer: customerPda,
          providerAuthority: provider.authority,
          provider: provider.publicKey,
          booking: bookingPda,
          escrow: escrowPda,
          systemProgram: SystemProgram.programId
        })
        .rpc();

      setStatus(`Booking created successfully! Transaction: ${signature}`);
    } catch (error) {
      console.error(error);
      setStatus(error.message || "Booking creation failed.");
    } finally {
      setBookingProvider(null);
    }
  }

  function formatPrice(price) {
    return (Number(price.toString()) / 1_000_000_000).toFixed(4);
  }

  return (
    <div>
      <h2>Service Marketplace</h2>

      <button onClick={loadProviders} disabled={loading || !program}>
        {loading ? "Loading..." : "Refresh Providers"}
      </button>

      {status && <p>{status}</p>}

      {!publicKey && <p>Connect your wallet to book a service.</p>}

      {providers.length === 0 && !loading && (
        <p>No registered providers found.</p>
      )}

      {providers.map((provider) => (
        <div key={provider.publicKey.toBase58()}>
          <hr />
          <h3>{provider.name}</h3>
          <p>Service: {provider.service}</p>
          <p>Price: {formatPrice(provider.priceLamports)} SOL</p>
          <p>
            Provider wallet: {provider.authority.toBase58()}
          </p>

          <button
            onClick={() => createBooking(provider)}
            disabled={
              !publicKey ||
              !program ||
              bookingProvider !== null ||
              provider.authority.equals(publicKey)
            }
          >
            {bookingProvider === provider.publicKey.toBase58()
              ? "Creating Booking..."
              : provider.authority.equals(publicKey)
                ? "Your Own Listing"
                : "Book Service"}
          </button>
        </div>
      ))}
    </div>
  );
}
