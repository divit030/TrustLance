
import { useMemo } from "react";
import { AnchorProvider, Program } from "@anchor-lang/core";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import idl from "../idl/project.json";

export function useServiceHubProgram() {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();

  return useMemo(() => {
    if (!wallet) return null;

    const provider = new AnchorProvider(connection, wallet, {
      commitment: "confirmed"
    });

    return new Program(idl, provider);
  }, [connection, wallet]);
}
