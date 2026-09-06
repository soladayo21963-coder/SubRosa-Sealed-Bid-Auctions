import "./globals.css";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "SubRosa Sealed Auctions",
  description: "Private sealed-bid auctions on Stellar Soroban",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
