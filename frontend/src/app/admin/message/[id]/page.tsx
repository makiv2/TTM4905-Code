import { notFound } from "next/navigation";
import Link from "next/link";
import Image from "next/image";

const messages = [
  {
    id: 3240,
    title: "Dumping oil into the North Sea",
    time: "2023-06-10T10:30:00",
    message: "How can I have one paragraph as a variable here",
    files: ["plan.pdf", "secret.jpg"],
    company: { name: "Statoil", logo: "/logos/statoil.png" },
  },
  {
    id: 1337,
    title: "Delaying trains on purpose",
    time: "2023-06-11T14:45:00",
    message: "Obviously we have been doing this for quite some time now",
    files: ["schedule.doc", "stamps.png"],
    company: { name: "NSB", logo: "/logos/nsb.png" },
  },
  // Add more dummy messages
];

export default function MessageDetailsPage({
  params,
}: {
  params: { id: string };
}) {
  const message = messages.find((msg) => msg.id === Number(params.id));

  if (!message) {
    notFound();
  }

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100">
      <div className="container mx-auto py-8">
        <h1 className="text-2xl font-bold mb-6 text-gray-800 text-center">
          {message.title}
        </h1>
        <div className="flex justify-center">
          <div className="w-full sm:w-4/5 md:w-3/4 lg:w-2/3 xl:w-1/2">
            <div className="bg-white shadow-md rounded-lg px-8 py-6 mb-4">
              <div className="mb-4">
                <span className="font-bold text-gray-700">ID:</span>{" "}
                <span className="text-gray-600">{message.id}</span>
              </div>
              <div className="mb-4">
                <span className="font-bold text-gray-700">Time:</span>{" "}
                <span className="text-gray-600">
                  {new Date(message.time).toLocaleString()}
                </span>
              </div>
              <div className="mb-4">
                <span className="font-bold text-gray-700">Company:</span>
                <div className="flex items-center">
                  <Image
                    src={message.company.logo}
                    alt={message.company.name}
                    width={20}
                    height={20}
                    className="w-6 h-6 mr-2"
                  />
                  <span className="text-gray-600">{message.company.name}</span>
                </div>
              </div>
              <div className="mb-4">
                <span className="font-bold text-gray-700">Message:</span>{" "}
                <span className="text-gray-600">{message.message}</span>
              </div>
              <div className="mb-4">
                <span className="font-bold text-gray-700">Files:</span>
                <ul className="list-disc list-inside text-gray-600">
                  {message.files.map((file, index) => (
                    <li key={index}>{file}</li>
                  ))}
                </ul>
              </div>
              <div className="mb-4">
                <span className="font-bold text-gray-700">Proof:</span>{" "}
                <span className="text-gray-600">proof.json</span>
              </div>
            </div>
            <Link href="/admin/dashboard">
              <div className="block bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded inline-flex items-center">
                <svg
                  className="w-4 h-4 mr-2"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    d="M26.105,21.891c-0.229,0-0.439-0.131-0.529-0.346l0,0c-0.066-0.156-1.716-3.857-7.885-4.59
		c-1.285-0.156-2.824-0.236-4.693-0.25v4.613c0,0.213-0.115,0.406-0.304,0.508c-0.188,0.098-0.413,0.084-0.588-0.033L0.254,13.815
		C0.094,13.708,0,13.528,0,13.339c0-0.191,0.094-0.365,0.254-0.477l11.857-7.979c0.175-0.121,0.398-0.129,0.588-0.029
		c0.19,0.102,0.303,0.295,0.303,0.502v4.293c2.578,0.336,13.674,2.33,13.674,11.674c0,0.271-0.191,0.508-0.459,0.562
		C26.18,21.891,26.141,21.891,26.105,21.891z"
                  />
                </svg>
                Back to Dashboard
              </div>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
