// app/layout.tsx
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "ZKVM Whistleblowing System",
  description: "System using SP1 ZKVM for whistleblowing",
  icons: {
    icon: "/logos/favicon.ico", // Use an absolute path for the favicon
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <link rel="icon" href="logos/favicon.ico" /> {/* Optional chaining */}
      </head>
      <body>{children}</body>
    </html>
  );
}
