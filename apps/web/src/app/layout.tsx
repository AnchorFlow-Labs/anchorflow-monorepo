import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "AnchorFlow",
  description: "Soroban-powered anchor flow dApp",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
