
import { useEffect, useState } from "react";
import { PublicKey } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import { useServiceHubProgram } from "../config/anchor";

const STATUS = {
  0: "Created",
  1: "Accepted",
  2: "Completed",
  3: "Approved",
  4: "Cancelled"
};

export default function ProviderDashboard() {
  const { publicKey } = useWallet();
  const program = useServiceHubProgram();

  const [bookings, setBookings] = useState([]);
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);

  async function loadBookings() {
    if (!program || !publicKey) return;

    try {
      setLoading(true);
      const [providerPda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode("provider"), publicKey.toBuffer()],
        program.programId
      );

      await program.account.provider.fetch(providerPda);

      const allBookings = await program.account.booking.all();

      setBookings(
        allBookings.filter((item) =>
          item.account.provider.equals(publicKey)
        )
      );

      setStatus("");
    } catch (error) {
      console.error(error);
      setStatus("Unable to load provider bookings. Make sure you are registered as a provider.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadBookings();
  }, [program, publicKey]);

  async function updateBooking(booking, action) {
    if (!program || !publicKey) return;

    try {
      setStatus("Sending transaction...");

      const [providerPda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode("provider"), publicKey.toBuffer()],
        program.programId
      );

      const method = action === "accept"
        ? program.methods.acceptBooking(booking.account.bookingId)
        : program.methods.completeJob(booking.account.bookingId);

      const signature = await method
        .accounts({
          providerAuthority: publicKey,
          provider: providerPda,
          booking: booking.publicKey
        })
        .rpc();

      setStatus(`Transaction successful: ${signature}`);
      await loadBookings();
    } catch (error) {
      console.error(error);
      setStatus(error.message || "Transaction failed.");
    }
  }

  if (!publicKey) {
    return <p>Connect your provider wallet first.</p>;
  }

  return (
    <div>
      <h2>Provider Dashboard</h2>

      <button onClick={loadBookings} disabled={loading}>
        {loading ? "Loading..." : "Refresh Bookings"}
      </button>

      {status && <p>{status}</p>}

      {bookings.length === 0 && !loading && (
        <p>No bookings found for this provider.</p>
      )}

      {bookings.map((item) => {
        const booking = item.account;
        const bookingKey = item.publicKey.toBase58();

        return (
          <div key={bookingKey}>
            <hr />
            <h3>{booking.service}</h3>
            <p>Booking ID: {booking.bookingId.toString()}</p>
            <p>Customer: {booking.customer.toBase58()}</p>
            <p>
              Amount: {(Number(booking.amountLamports.toString()) / 1_000_000_000).toFixed(4)} SOL
            </p>
            <p>Status: {STATUS[booking.status] || "Unknown"}</p>

            {booking.status === 0 && (
              <button onClick={() => updateBooking(item, "accept")}>
                Accept Booking
              </button>
            )}

            {booking.status === 1 && (
              <button onClick={() => updateBooking(item, "complete")}>
                Mark Job Completed
              </button>
            )}
          </div>
        );
      })}
    </div>
  );
}
