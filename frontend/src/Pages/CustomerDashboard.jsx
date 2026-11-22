
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

export default function CustomerDashboard() {
  const { publicKey } = useWallet();
  const program = useServiceHubProgram();

  const [bookings, setBookings] = useState([]);
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);

  async function loadBookings() {
    if (!program || !publicKey) return;

    try {
      setLoading(true);

      const [customerPda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode("customer"), publicKey.toBuffer()],
        program.programId
      );

      await program.account.customer.fetch(customerPda);

      const allBookings = await program.account.booking.all();

      setBookings(
        allBookings.filter((item) =>
          item.account.customer.equals(publicKey)
        )
      );

      setStatus("");
    } catch (error) {
      console.error(error);
      setStatus("Unable to load bookings. Make sure you are registered as a customer.");
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadBookings();
  }, [program, publicKey]);

  async function performAction(item, action) {
    if (!program || !publicKey) return;

    try {
      setStatus("Sending transaction...");

      const booking = item.account;

      const [customerPda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode("customer"), publicKey.toBuffer()],
        program.programId
      );

      const [escrowPda] = PublicKey.findProgramAddressSync(
        [
          new TextEncoder().encode("escrow"),
          item.publicKey.toBuffer()
        ],
        program.programId
      );

      let signature;

      if (action === "approve") {
        signature = await program.methods
          .approveJob(booking.bookingId)
          .accounts({
            customerAuthority: publicKey,
            customer: customerPda,
            providerAuthority: booking.provider,
            booking: item.publicKey,
            escrow: escrowPda
          })
          .rpc();
      }

      if (action === "cancel") {
        signature = await program.methods
          .cancelBooking(booking.bookingId)
          .accounts({
            customerAuthority: publicKey,
            customer: customerPda,
            booking: item.publicKey,
            escrow: escrowPda
          })
          .rpc();
      }

      setStatus(`Transaction successful: ${signature}`);
      await loadBookings();
    } catch (error) {
      console.error(error);
      setStatus(error.message || "Transaction failed.");
    }
  }

  if (!publicKey) {
    return <p>Connect your customer wallet first.</p>;
  }

  return (
    <div>
      <h2>Customer Dashboard</h2>

      <button onClick={loadBookings} disabled={loading}>
        {loading ? "Loading..." : "Refresh Bookings"}
      </button>

      {status && <p>{status}</p>}

      {bookings.length === 0 && !loading && (
        <p>You have no bookings yet.</p>
      )}

      {bookings.map((item) => {
        const booking = item.account;

        return (
          <div key={item.publicKey.toBase58()}>
            <hr />
            <h3>{booking.service}</h3>
            <p>Booking ID: {booking.bookingId.toString()}</p>
            <p>Provider: {booking.provider.toBase58()}</p>
            <p>
              Amount: {(Number(booking.amountLamports.toString()) / 1_000_000_000).toFixed(4)} SOL
            </p>
            <p>Status: {STATUS[booking.status] || "Unknown"}</p>

            {booking.status === 2 && (
              <button onClick={() => performAction(item, "approve")}>
                Approve Job and Release Payment
              </button>
            )}

            {booking.status === 0 && (
              <button onClick={() => performAction(item, "cancel")}>
                Cancel Booking and Refund
              </button>
            )}
          </div>
        );
      })}
    </div>
  );
}
