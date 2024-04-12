// app/page.tsx
"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import CompanyDropdown from "./components/CompanyDropdown"; // Adjust the path as needed
import { sha512 } from "js-sha512";

const companies = [
  { name: "Statoil", logo: "/logos/statoil.png" },
  { name: "NSB", logo: "/logos/nsb.png" },
  { name: "Statkraft", logo: "/logos/statkraft.png" },
  // Add more companies with their logos
];

export default function LoginPage() {
  const [selectedCompany, setSelectedCompany] = useState(companies[0]);
  const [searchTerm, setSearchTerm] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    try {
      const hashedUsername = sha512(email);
      console.log(hashedUsername);

      const response = await fetch("http://0.0.0.0:9999/check_credentials", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          company: selectedCompany.name,
          username: hashedUsername,
          password,
        }),
      });

      if (response.ok) {
        router.push("/dashboard");
      } else {
        // Handle login error
        console.error("Login failed");
        router.push("/");
      }
    } catch (error) {
      console.error("Error:", error);
    }
  };

  const filteredCompanies = companies.filter((company) =>
    company.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100">
      <div className="w-full max-w-md">
        <form
          onSubmit={handleSubmit}
          className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
        >
          <div className="mb-4">
            <label className="block text-gray-700 font-bold mb-2">
              Company
            </label>
            <CompanyDropdown
              companies={companies}
              selectedCompany={selectedCompany}
              onChange={setSelectedCompany}
            />
          </div>
          <div className="mb-4">
            <label
              className="block text-gray-700 font-bold mb-2"
              htmlFor="email"
            >
              Email
            </label>
            <input
              type="email"
              id="email"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
          </div>
          <div className="mb-6">
            <label
              className="block text-gray-700 font-bold mb-2"
              htmlFor="password"
            >
              Password
            </label>
            <input
              type="password"
              id="password"
              className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
          </div>
          <div className="flex items-center justify-between">
            <button
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit"
            >
              Sign In
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
