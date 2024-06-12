"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import CompanyDropdown from "../components/CompanyDropdown"; // Adjust the path as needed

// Dummy companies for appearance
const companies = [
  { name: "Statoil", logo: "/logos/Statoil.png" },
  { name: "NSB", logo: "/logos/NSB.png" },
  { name: "Statkraft", logo: "/logos/Statkraft.png" },
  // Add more companies with their logos
];

export default function DashboardPage() {
  const [selectedCompany, setSelectedCompany] = useState(companies[0]);
  const [messageFile, setMessageFile] = useState<File | null>(null);
  const [signatureFile, setSignatureFile] = useState<File | null>(null);
  const [showSuccessMessage, setShowSuccessMessage] = useState(false);
  const [showFailureMessage, setShowFailureMessage] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const router = useRouter();

  const handleFileChange = (
    event: React.ChangeEvent<HTMLInputElement>,
    setFile: React.Dispatch<React.SetStateAction<File | null>>
  ) => {
    const file = event.target.files?.[0] || null;
    setFile(file);
  };

  const readFileAsBase64 = (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = (error) => reject(error);
      reader.readAsDataURL(file); // Reads file as data URL
    });
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      if (!messageFile || !signatureFile) {
        throw new Error("Both files must be uploaded.");
      }

      const messageFileBase64Url = await readFileAsBase64(messageFile);
      const signatureFileBase64Url = await readFileAsBase64(signatureFile);
      const companyBase64 = btoa(selectedCompany.name);

      // Remove the data URL prefix
      const messageFileBase64 = messageFileBase64Url.split("base64,")[1];
      const signatureFileBase64 = signatureFileBase64Url.split("base64,")[1];

      const response = await fetch("http://localhost:9999/generate_proof", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          companyb64: companyBase64,
          messageb64: messageFileBase64,
          signatureb64: signatureFileBase64,
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
      setShowFailureMessage(true);
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
              <p>Failed to generate proof.</p>
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
              <label className="block text-gray-700 font-bold mb-2">
                Message File
              </label>
              <input
                type="file"
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                onChange={(e) => handleFileChange(e, setMessageFile)}
              />
            </div>
            <div className="mb-4">
              <label className="block text-gray-700 font-bold mb-2">
                Signature File
              </label>
              <input
                type="file"
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                onChange={(e) => handleFileChange(e, setSignatureFile)}
              />
            </div>
            <div className="flex items-center justify-between">
              <button
                type="submit"
                className={`${
                  isSubmitting ? "bg-gray-500" : "bg-blue-500"
                } hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline`}
                disabled={isSubmitting}
              >
                {isSubmitting ? "Done" : "Submit"}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
