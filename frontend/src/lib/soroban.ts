import {
  Address,
  BASE_FEE,
  Contract,
  Networks,
  Server,
  TransactionBuilder,
  nativeToScVal,
  xdr,
} from "@stellar/stellar-sdk";
import { signTransaction } from "@stellar/freighter-api";
import { Buffer } from "buffer";

const networkPassphrase = Networks.TESTNET;
const rpcUrl = "https://soroban-testnet.stellar.org";
const horizonUrl = "https://horizon-testnet.stellar.org";

function getContractId() {
  const contractId = process.env.NEXT_PUBLIC_SUBROSA_CONTRACT_ID;
  if (!contractId) {
    throw new Error("NEXT_PUBLIC_SUBROSA_CONTRACT_ID is not configured.");
  }
  return contractId;
}

function toBytes32(value: Uint8Array) {
  return xdr.ScVal.scvBytes(Buffer.from(value));
}

export async function createCommitment(bidAmount: bigint, salt: Uint8Array, auctionId: bigint) {
  if (bidAmount <= BigInt(0) || bidAmount > (BigInt(1) << BigInt(127)) - BigInt(1)) {
    throw new Error("Bid amount is outside the supported i128 range.");
  }
  const amount = new Uint8Array(16);
  const view = new DataView(amount.buffer);
  const high = bidAmount >> BigInt(64);
  const low = bidAmount & ((BigInt(1) << BigInt(64)) - BigInt(1));
  view.setBigInt64(0, high);
  view.setBigUint64(8, low);
  const auction = new Uint8Array(8);
  new DataView(auction.buffer).setBigUint64(0, auctionId);
  const payload = new Uint8Array(amount.length + salt.length + auction.length);
  payload.set(amount, 0);
  payload.set(salt, amount.length);
  payload.set(auction, amount.length + salt.length);
  return new Uint8Array(await crypto.subtle.digest("SHA-256", payload));
}

async function submitContractCall(accountId: string, operation: xdr.Operation) {
  const server = new Server(rpcUrl);
  const account = await server.getAccount(accountId);
  const transaction = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase,
  })
    .addOperation(operation)
    .setTimeout(180)
    .build();
  const simulation = await server.simulateTransaction(transaction);
  if ("error" in simulation) {
    throw new Error(simulation.error);
  }
  const prepared = Server.assembleTransaction(transaction, simulation).build();
  const signed = await signTransaction(prepared.toXDR(), {
    networkPassphrase,
  });
  const result = await server.sendTransaction(
    TransactionBuilder.fromXDR(signed.signedTxXdr, networkPassphrase),
  );
  if (result.status === "ERROR") {
    throw new Error("Soroban transaction was rejected.");
  }

  for (let attempt = 0; attempt < 30; attempt += 1) {
    const transaction = await server.getTransaction(result.hash);
    if (transaction.status === "SUCCESS") return result.hash;
    if (transaction.status === "FAILED") {
      throw new Error("Soroban transaction failed on-chain.");
    }
    await new Promise((resolve) => setTimeout(resolve, 2_000));
  }

  throw new Error("Transaction submitted but confirmation timed out.");
}

export async function submitSealedBid(
  accountId: string,
  auctionId: bigint,
  commitment: Uint8Array,
  collateral: bigint,
) {
  const contract = new Contract(getContractId());
  return submitContractCall(
    accountId,
    contract.call(
      "submit_sealed_bid",
      new Address(accountId).toScVal(),
      nativeToScVal(auctionId, { type: "u64" }),
      toBytes32(commitment),
      nativeToScVal(collateral, { type: "i128" }),
    ),
  );
}

function fromHex(value: string) {
  if (!/^[0-9a-f]{64}$/i.test(value)) {
    throw new Error("Reveal salt must be exactly 32 bytes in hexadecimal form.");
  }
  return Uint8Array.from(value.match(/.{2}/g)!, (byte) => parseInt(byte, 16));
}

export async function revealBid(
  accountId: string,
  auctionId: bigint,
  bidAmount: bigint,
  saltHex: string,
) {
  const contract = new Contract(getContractId());
  return submitContractCall(
    accountId,
    contract.call(
      "reveal_bid",
      new Address(accountId).toScVal(),
      nativeToScVal(auctionId, { type: "u64" }),
      nativeToScVal(bidAmount, { type: "i128" }),
      toBytes32(fromHex(saltHex)),
    ),
  );
}
