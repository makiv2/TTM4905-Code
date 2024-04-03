// app/admin/page.tsx
"use client";

import { useState } from "react";

interface Post {
  id: number;
  title: string;
  time: string;
  message: string;
  files: string[];
}

const mockPosts: Post[] = [
  {
    id: 1,
    title: "Post 1",
    time: "2023-06-10 10:00:00",
    message: "This is the message for Post 1.",
    files: ["file1.pdf", "file2.jpg"],
  },
  {
    id: 2,
    title: "Post 2",
    time: "2023-06-11 14:30:00",
    message: "This is the message for Post 2.",
    files: ["file3.docx"],
  },
  // Add more mock posts as needed
];

export default function AdminPage() {
  const [selectedPost, setSelectedPost] = useState<Post | null>(null);

  const handlePostClick = (post: Post) => {
    setSelectedPost(post);
  };

  return (
    <div className="flex justify-center items-center min-h-screen bg-gray-100">
      <div className="w-full max-w-4xl">
        <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
          <h1 className="text-2xl font-bold mb-6 text-gray-800">Admin Panel</h1>
          <div className="grid grid-cols-2 gap-8">
            <div>
              <h2 className="text-xl font-bold mb-4 text-gray-800">
                Posted Messages
              </h2>
              <ul className="space-y-4">
                {mockPosts.map((post) => (
                  <li
                    key={post.id}
                    className="cursor-pointer bg-gray-100 hover:bg-gray-200 p-4 rounded"
                    onClick={() => handlePostClick(post)}
                  >
                    <div className="font-bold text-gray-800">{post.title}</div>
                    <div className="text-sm text-gray-600">{post.time}</div>
                    <div className="text-sm text-gray-600">ID: {post.id}</div>
                  </li>
                ))}
              </ul>
            </div>
            <div>
              <h2 className="text-xl font-bold mb-4 text-gray-800">
                Post Details
              </h2>
              {selectedPost ? (
                <div className="bg-gray-100 p-4 rounded">
                  <div className="mb-2">
                    <span className="font-bold text-gray-800">Title:</span>{" "}
                    {selectedPost.title}
                  </div>
                  <div className="mb-2">
                    <span className="font-bold text-gray-800">Time:</span>{" "}
                    {selectedPost.time}
                  </div>
                  <div className="mb-2">
                    <span className="font-bold text-gray-800">ID:</span>{" "}
                    {selectedPost.id}
                  </div>
                  <div className="mb-2">
                    <span className="font-bold text-gray-800">Message:</span>{" "}
                    {selectedPost.message}
                  </div>
                  <div>
                    <span className="font-bold text-gray-800">Files:</span>
                    <ul className="ml-4 list-disc text-gray-700">
                      {selectedPost.files.map((file, index) => (
                        <li key={index}>{file}</li>
                      ))}
                    </ul>
                  </div>
                </div>
              ) : (
                <div className="text-gray-600">No post selected.</div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
