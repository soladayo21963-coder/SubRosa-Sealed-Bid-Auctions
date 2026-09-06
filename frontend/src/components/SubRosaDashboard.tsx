"use client";

import React, { useState } from "react";
import { connectWallet } from "@/lib/freighter";
import { createCommitment, submitSealedBid } from "@/lib/soroban";

export default function SubRosaDashboard() {
  const [account, setAccount] = useState<string | null>(null);
  const [collateral, setCollateral] = useState("");
  const [auctionId, setAuctionId] = useState("1");
  const [bidAmount, setBidAmount] = useState("");
  const [message, setMessage] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleConnect = async () => {
    const addr = await connectWallet();
    if (addr) {
      setAccount(addr);
    }
  };

  const handleBid = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setMessage(null);

    try {
      if (!account) {
        throw new Error("Connect Freighter before submitting a bid.");
      }
      const amount = BigInt(bidAmount);
      const locked = BigInt(collateral);
      if (amount <= BigInt(0) || locked < amount) {
        throw new Error("Collateral must be greater than or equal to the bid.");
      }
      const salt = crypto.getRandomValues(new Uint8Array(32));
      const commitment = await createCommitment(amount, salt);
      localStorage.setItem(
        `subrosa:salt:${auctionId}:${account}`,
        Array.from(salt, (byte) => byte.toString(16).padStart(2, "0")).join(""),
      );
      const hash = await submitSealedBid(account, BigInt(auctionId), commitment, locked);
      setMessage(`Commitment submitted. Transaction: ${hash}`);
    } catch (error) {
      setMessage(error instanceof Error ? error.message : "Unable to submit commitment.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="min-h-screen bg-slate-950 text-white flex items-center justify-center p-6">
      <div className="w-full max-w-2xl bg-slate-900 border border-slate-800 rounded-2xl p-8 shadow-2xl shadow-rose-950/30">
        <div className="flex justify-between items-center mb-6 border-b border-slate-800 pb-4">
          <div>
            <h1 className="text-2xl font-bold text-rose-400">SubRosa Sealed Auctions</h1>
            <p className="text-xs text-slate-400">MEV-resistant private bidding infrastructure</p>
          </div>
          <span className="px-3 py-1 bg-rose-950 text-rose-400 text-xs font-mono rounded-full border border-rose-800">
            Soroban Testnet
          </span>
        </div>

        {!account ? (
          <button
            onClick={handleConnect}
            className="w-full py-4 bg-rose-600 hover:bg-rose-500 text-white font-bold rounded-xl transition shadow-lg shadow-rose-600/20"
          >
            Connect Freighter Wallet
          </button>
        ) : (
          <form onSubmit={handleBid} className="space-y-6">
            <div>
              <label className="block text-xs font-semibold text-slate-400 uppercase mb-2">Auction ID</label>
              <input
                type="number"
                min="1"
                required
                value={auctionId}
                onChange={(e) => setAuctionId(e.target.value)}
                className="w-full p-3 bg-slate-950 border border-slate-800 rounded-lg text-sm font-mono text-white focus:outline-none focus:border-rose-500"
              />
            </div>

            <div>
              <label className="block text-xs font-semibold text-slate-400 uppercase mb-2">Sealed Bid (stroops)</label>
              <input
                type="number"
                min="1"
                required
                value={bidAmount}
                onChange={(e) => setBidAmount(e.target.value)}
                placeholder="75000000"
                className="w-full p-3 bg-slate-950 border border-slate-800 rounded-lg text-sm font-mono text-white focus:outline-none focus:border-rose-500"
              />
            </div>

            <div>
              <label className="block text-xs font-semibold text-slate-400 uppercase mb-2">Bidder Address</label>
              <div className="p-3 bg-slate-950 border border-slate-800 rounded-lg text-xs font-mono text-rose-300 truncate">
                {account}
              </div>
            </div>

            <div>
              <label className="block text-xs font-semibold text-slate-400 uppercase mb-2">Escrow Collateral (stroops)</label>
              <input
                type="number"
                required
                value={collateral}
                onChange={(e) => setCollateral(e.target.value)}
                placeholder="100000000"
                className="w-full p-3 bg-slate-950 border border-slate-800 rounded-lg text-sm font-mono text-white focus:outline-none focus:border-rose-500"
              />
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full py-4 bg-rose-600 hover:bg-rose-500 text-white font-bold rounded-xl transition shadow-lg shadow-rose-600/20 disabled:opacity-60"
            >
              {loading ? "Submitting Commitment..." : "Submit Sealed Bid"}
            </button>
            {message && <p className="text-xs text-emerald-300 break-words">{message}</p>}
          </form>
        )}
      </div>
    </main>
  );
}
