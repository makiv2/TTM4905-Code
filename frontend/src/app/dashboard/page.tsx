// app/dashboard/page.tsx
"use client";

import { useState } from "react";

export default function DashboardPage() {
  const [showSuccessMessage, setShowSuccessMessage] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setIsSubmitting(true);
    // Perform form submission logic here
    setShowSuccessMessage(true);
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
          <form onSubmit={handleSubmit}>
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
                disabled={isSubmitting}
              ></textarea>
            </div>
            <div className="mb-6">
              <label
                className="block text-gray-700 font-bold mb-2"
                htmlFor="files"
              >
                Files
              </label>
              <input
                type="file"
                id="files"
                className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                multiple
                disabled={isSubmitting}
              />
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
