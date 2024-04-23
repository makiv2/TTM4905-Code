"use client";

import Link from "next/link";
import Image from "next/image";
import { useState, useEffect } from "react";
import { log } from "console";

interface Proof {
  company: string;
  id: number;
  match: boolean;
  message: string;
}

const messages = [
  {
    id: 3240,
    title: "Dumping oil into the North Sea",
    time: "2023-06-10T10:30:00",
    company: { name: "Statoil", logo: "/logos/statoil.png" },
  },
  {
    id: 1337,
    title: "Delaying trains on p",
    time: "2023-06-11T14:45:00",
    company: { name: "NSB", logo: "/logos/nsb.png" },
  },
  // Add more dummy messages
];

export default function AdminDashboardPage() {
  const [proofs, setProofs] = useState<Proof[]>([]);

  useEffect(() => {
    const fetchProofs = async () => {
      try {
        const response = await fetch("http://localhost:9999/proofs");
        const data: Proof[] = await response.json();
        setProofs(data);
      } catch (error) {
        console.error("Error fetching proofs:", error);
      }
    };

    fetchProofs();
  }, []);
  console.log("proofs");
  console.log(proofs);

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-200">
      <div className="container mx-auto py-8">
        <h1 className="text-3xl font-bold mb-6 text-gray-800 text-center">
          Whistleblowing Portal
        </h1>
        <div className="flex justify-center">
          <div className="overflow-x-auto">
            <table className="bg-white shadow-md rounded-lg mb-4">
              <thead>
                <tr className="bg-gray-100">
                  <th className="py-3 px-6 font-semibold text-sm text-gray-600 border-b border-gray-200 text-left">
                    ID
                  </th>
                  <th className="py-3 px-6 font-semibold text-sm text-gray-600 border-b border-gray-200 text-left">
                    Title
                  </th>
                  <th className="py-3 px-6 font-semibold text-sm text-gray-600 border-b border-gray-200 text-left">
                    Time
                  </th>
                  <th className="py-3 px-6 font-semibold text-sm text-gray-600 border-b border-gray-200 text-left">
                    Company
                  </th>
                </tr>
              </thead>
              <tbody>
                {proofs.map((proof) => (
                  <tr key={proof.id} className="hover:bg-gray-50">
                    <td className="py-4 px-6 border-b border-gray-200 text-left text-black">
                      <Link href={`/admin/message/${proof.id}`}>
                        {proof.id}
                      </Link>
                    </td>
                    <td className="py-4 px-6 border-b border-gray-200 text-left text-black">
                      {proof.message}
                    </td>
                    <td className="py-4 px-6 border-b border-gray-200 text-left text-black">
                      {new Date(messages[1].time).toLocaleString()}
                    </td>
                    <td className="py-4 px-6 border-b border-gray-200 text-black">
                      <div className="flex items-center">
                        <Image
                          src={`/logos/${proof.company}.png`}
                          alt={proof.company}
                          width={20}
                          height={20}
                          className="w-6 h-6 mr-2 rounded-full"
                        />
                        <span className="text-sm text-gray-800">
                          {proof.company}
                        </span>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
