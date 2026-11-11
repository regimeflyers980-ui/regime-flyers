"use client";

import { useEffect, useState } from "react";

interface Demo {
  system: string;
  status: string;
  demo_url: string;
}

export default function Home() {
  const [demos, setDemos] = useState<Demo[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Fetch from the Rust backend
    fetch("http://192.168.114.135:3001/api/demos")
      .then((res) => res.json())
      .then((data) => {
        setDemos(data);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to fetch demos:", err);
        setLoading(false);
      });
  }, []);

  return (
    <main className="min-h-screen bg-gray-50 px-4 py-12 sm:px-6 lg:px-8">
      <div className="mx-auto max-w-3xl text-center">
        <h1 className="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
          REGIME FLYERS
        </h1>
        <p className="mt-4 text-lg text-gray-600 sm:text-xl">
          Secure, high-performance information systems. Built in Rust, delivered
          to you.
        </p>
      </div>

      <div className="mx-auto mt-12 max-w-2xl">
        <h2 className="text-2xl font-semibold text-gray-900">Available Demos</h2>
        {loading && <p className="mt-2 text-gray-500">Loading demos...</p>}
        
        <ul className="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
          {demos.map((demo) => (
            <li
              key={demo.system}
              className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm transition hover:shadow-md"
            >
              <h3 className="text-lg font-medium text-gray-900">
                {demo.system}
              </h3>
              <p className="mt-1 text-sm text-gray-500">Status: {demo.status}</p>
              <button
                onClick={() => window.open(demo.demo_url, "_blank")}
                className="mt-4 inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
              >
                Access Demo
              </button>
            </li>
          ))}
        </ul>
      </div>
    </main>
  );
}
