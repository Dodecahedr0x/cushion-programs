import { PublicKey } from "@solana/web3.js";
import { CUSHION_PROGRAM_ID } from "./constants";

export function getLlammaKey(stablecoin: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [stablecoin.toBuffer()],
    CUSHION_PROGRAM_ID
  )[0];
}

export function getLlammaAuthorityKey(stablecoin: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [stablecoin.toBuffer(), Buffer.from("authority")],
    CUSHION_PROGRAM_ID
  )[0];
}

export function getMarketKey(llamma: PublicKey, collateral: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [llamma.toBuffer(), collateral.toBuffer()],
    CUSHION_PROGRAM_ID
  )[0];
}
