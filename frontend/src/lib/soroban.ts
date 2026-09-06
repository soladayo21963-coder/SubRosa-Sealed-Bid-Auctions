import {
  Address,
  BASE_FEE,
  Contract,
  Networks,
  TransactionBuilder,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { assembleTransaction, Server } from "@stellar/stellar-sdk/rpc";
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
  const prepared = assembleTransaction(transaction, simulation).build();
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

async function simulateContractCall(accountId: string, operation: xdr.Operation) {
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
  if ("error" in simulation) throw new Error(simulation.error);
  if (!simulation.result) throw new Error("Soroban simulation returned no result.");
  return scValToNative(simulation.result.retval);
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

export async function createAuction(
  accountId: string,
  assetToken: string,
  assetAmount: bigint,
  bidDeadline: bigint,
  revealDeadline: bigint,
) {
  const contract = new Contract(getContractId());
  return submitContractCall(
    accountId,
    contract.call(
      "create_auction",
      new Address(accountId).toScVal(),
      new Address(assetToken).toScVal(),
      nativeToScVal(assetAmount, { type: "i128" }),
      nativeToScVal(bidDeadline, { type: "u64" }),
      nativeToScVal(revealDeadline, { type: "u64" }),
    ),
  );
}

export async function getAuction(accountId: string, auctionId: bigint) {
  const contract = new Contract(getContractId());
  return simulateContractCall(
    accountId,
    contract.call("get_auction", nativeToScVal(auctionId, { type: "u64" })),
  );
}

export async function discoverAuctions(accountId: string, maximum = 20) {
  const auctions = [];
  for (let id = BigInt(1); id <= BigInt(maximum); id += BigInt(1)) {
    try {
      auctions.push(await getAuction(accountId, id));
    } catch {
      break;
    }
  }
  return auctions;
}

export async function finalizeAuction(accountId: string, auctionId: bigint) {
  const contract = new Contract(getContractId());
  return submitContractCall(
    accountId,
    contract.call("finalize_auction", nativeToScVal(auctionId, { type: "u64" })),
  );
}

export async function claimRefund(accountId: string, auctionId: bigint) {
  const contract = new Contract(getContractId());
  return submitContractCall(
    accountId,
    contract.call(
      "claim_refund",
      nativeToScVal(auctionId, { type: "u64" }),
      new Address(accountId).toScVal(),
    ),
  );
}
