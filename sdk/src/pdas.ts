import { PublicKey } from "@solana/web3.js";
import { CUSHION_PROGRAM_ID } from "./constants";
import { BN } from "bn.js";

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

export function getBandKey(market: PublicKey, index: number) {
  return PublicKey.findProgramAddressSync(
    [market.toBuffer(), new BN(index).toArrayLike(Buffer, "le", 2)],
    CUSHION_PROGRAM_ID
  )[0];
}

export function getBandDepositKey(band: PublicKey, depositor: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [band.toBuffer(), depositor.toBuffer()],
    CUSHION_PROGRAM_ID
  )[0];
}
