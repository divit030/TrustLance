
import { useEffect, useState } from "react";
import { SystemProgram } from "@solana/web3.js";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useServiceHubProgram } from "../config/anchor";

export default function RegisterCustomer() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const program = useServiceHubProgram();

  const [name, setName] = useState("");
  const [checking, setChecking] = useState(true);
  const [registered, setRegistered] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  async function checkRegistration() {
    if (!publicKey || !program) {
      setChecking(false);
      setRegistered(false);
      return;
    }

    try {
      setChecking(true);
      setError("");

      const [customerPda] = getCustomerPda();

      const accountInfo = await connection.getAccountInfo(
        customerPda,
        "confirmed"
      );

      setRegistered(
        accountInfo !== null && accountInfo.owner.equals(program.programId)
      );
    } catch (err) {
      console.error("Customer registration check failed:", err);
      setError("Unable to check registration. Please try again.");
    } finally {
      setChecking(false);
    }
  }

  function getCustomerPda() {
    return window.solana
      ? [
          program.programId.constructor.findProgramAddressSync(
            [
              new TextEncoder().encode("customer"),
              publicKey.toBuffer()
            ],
            program.programId
          )[0]
        ]
      : [null];
  }

  useEffect(() => {
    let active = true;

    async function check() {
      if (!publicKey || !program) {
        setRegistered(false);
        setChecking(false);
        return;
      }

      try {
        setChecking(true);
        setError("");

        const [customerPda] = program.programId.constructor.findProgramAddressSync(
          [
            new TextEncoder().encode("customer"),
            publicKey.toBuffer()
          ],
          program.programId
        );

        const accountInfo = await connection.getAccountInfo(
          customerPda,
          "confirmed"
        );

        if (active) {
          setRegistered(
            accountInfo !== null && accountInfo.owner.equals(program.programId)
          );
        }
      } catch (err) {
        console.error(err);
        if (active) {
          setError("Unable to check registration. Please try again.");
        }
      } finally {
        if (active) {
          setChecking(false);
        }
      }
    }

    check();

    return () => {
      active = false;
    };
  }, [connection, publicKey, program]);

  async function handleRegister(event) {
    event.preventDefault();

    if (!publicKey || !program || !name.trim()) {
      setError("Please connect your wallet and enter your name.");
      return;
    }

    try {
      setSubmitting(true);
      setError("");
      setMessage("Please approve the transaction in Phantom.");

      const [customerPda] = program.programId.constructor.findProgramAddressSync(
        [
          new TextEncoder().encode("customer"),
          publicKey.toBuffer()
        ],
        program.programId
      );

      const signature = await program.methods
        .registerCustomer(name.trim())
        .accounts({
          authority: publicKey,
          customer: customerPda,
          systemProgram: SystemProgram.programId
        })
        .rpc();

      setRegistered(true);
      setMessage(`Customer registered successfully! Transaction: ${signature}`);
      setName("");
    } catch (err) {
      console.error("Customer registration failed:", err);
      setMessage("");
      setError(err.message || "Registration failed. Please try again.");
    } finally {
      setSubmitting(false);
    }
  }

  if (!publicKey) {
    return (
      <div className="registration-card">
        <h2>Customer Registration</h2>
        <p>Please connect your Phantom wallet to continue.</p>
      </div>
    );
  }

  if (checking) {
    return (
      <div className="registration-card">
        <p>Checking your customer registration on Solana...</p>
      </div>
    );
  }

  if (error && !registered && !submitting) {
    return (
      <div className="registration-card">
        <h2>Customer Registration</h2>
        <p className="error-message">{error}</p>
        <button onClick={checkRegistration}>Try Again</button>
      </div>
    );
  }

  if (registered) {
    return (
      <div className="registration-card">
        <div className="registration-success-icon">✓</div>
        <span className="registration-tag">CUSTOMER ACCOUNT</span>
        <h2>Already Registered!</h2>
        <p>
          Your connected wallet is already registered as a customer.
        </p>
        <div className="wallet-address">
          <strong>Wallet Address</strong>
          <span>{publicKey.toBase58()}</span>
        </div>
        <button onClick={() => window.location.reload()}>
          Refresh Registration
        </button>
      </div>
    );
  }

  return (
    <div className="registration-card">
      <span className="registration-tag">GET STARTED</span>
      <h2>Register as a Customer</h2>
      <p>Create your customer account to start booking services.</p>

      <form onSubmit={handleRegister}>
        <label htmlFor="customer-name">Your Name</label>
        <input
          id="customer-name"
          type="text"
          placeholder="Enter your full name"
          value={name}
          onChange={(event) => setName(event.target.value)}
          maxLength={50}
          required
        />

        <button type="submit" disabled={submitting || !name.trim()}>
          {submitting ? "Registering..." : "Register Customer"}
        </button>
      </form>

      {message && <p className="success-message">{message}</p>}
      {error && <p className="error-message">{error}</p>}
    </div>
  );
}
