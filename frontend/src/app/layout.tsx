import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Regime Flyers",
  description: "Secure, high-performance information systems.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased font-sans">
        {children}
      </body>
    </html>
  );
}
