"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import CompanyDropdown from "../components/CompanyDropdown"; // Adjust the path as needed

const companies = [
  { name: "Statoil", logo: "/logos/Statoil.png" },
  { name: "NSB", logo: "/logos/NSB.png" },
  { name: "Statkraft", logo: "/logos/Statkraft.png" },
  // Add more companies with their logos
];

export default function DashboardPage() {
  const [selectedCompany, setSelectedCompany] = useState(companies[0]);
  const [searchTerm, setSearchTerm] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const [showSuccessMessage, setShowSuccessMessage] = useState(false);
  const [showFailureMessage, setShowFailureMessage] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      const response = await fetch("http://localhost:9999/generate_proof", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username: email,
          password: password,
          company: selectedCompany.name,
          message: message,
        }),
      });
      const data = await response.text();
      console.log(data);

      if (data != "Failure." && response.ok) {
        setShowSuccessMessage(true);
      } else {
        // Handle submission error
        setShowFailureMessage(true);
        console.error("Submission failed");
      }
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100">
      <div className="w-full max-w-xl">
        <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
          <h1 className="text-2xl font-bold mb-6">Dashboard</h1>
          {showSuccessMessage && (
            <div
              className="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-6"
              role="alert"
            >
              <p className="font-bold">Success!</p>
              <p>Your message has been posted successfully.</p>
            </div>
          )}
          {showFailureMessage && (
            <div
              className="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-6"
              role="alert"
            >
              <p className="font-bold">Failure!</p>
              <p>Failed to generate proof </p>
            </div>
          )}
          <form onSubmit={handleSubmit}>
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
            <div className="mb-4">
              <label
                className="block text-gray-700 font-bold mb-2"
                htmlFor="message"
              >
                Message
              </label>
              <textarea
                id="message"
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder="Enter your message"
                rows={4}
                value={message}
                onChange={(e) => setMessage(e.target.value)}
                required
              ></textarea>
            </div>
            <div className="flex items-center justify-between">
              <button
                className={`${
                  isSubmitting
                    ? "bg-gray-400 cursor-not-allowed"
                    : "bg-blue-500 hover:bg-blue-700"
                } text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline`}
                type="submit"
                disabled={isSubmitting}
              >
                {isSubmitting ? "Done" : "Post Message"}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
