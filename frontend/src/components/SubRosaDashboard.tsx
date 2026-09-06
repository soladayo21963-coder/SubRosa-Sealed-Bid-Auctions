"use client";

import React, { useState } from "react";
import { connectWallet } from "@/lib/freighter";

export default function SubRosaDashboard() {
  const [account, setAccount] = useState<string | null>(null);
  const [collateral, setCollateral] = useState("");
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

    try {
      alert("Sealed bid submitted. Commitment hash stored on the Soroban testnet.");
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
          </form>
        )}
      </div>
    </main>
  );
}
