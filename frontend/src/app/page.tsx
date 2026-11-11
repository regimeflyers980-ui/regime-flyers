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
    <main className="min-h-screen bg-slate-50 font-sans">
      {/* Header Section with Gradient */}
      <div className="bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] py-20 px-4 sm:px-6 lg:px-8 shadow-lg">
        <div className="mx-auto max-w-4xl text-center">
          <h1 className="text-4xl font-extrabold tracking-tight text-white sm:text-5xl md:text-6xl drop-shadow-md">
            REGIME FLYERS
          </h1>
          <p className="mt-6 text-lg text-blue-50 sm:text-xl max-w-2xl mx-auto">
            Secure, high-performance information systems. Architected in Rust, delivered to you.
          </p>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="mx-auto mt-16 max-w-4xl px-4 sm:px-6 lg:px-8 pb-20">
        <div className="flex items-center justify-between mb-8">
          <h2 className="text-2xl font-bold text-slate-800">
            Available Systems
          </h2>
          <div className="h-1 flex-grow mx-4 bg-gradient-to-r from-[#4682B4]/20 to-[#8A2BE2]/20 rounded-full"></div>
        </div>

        {loading && (
          <div className="text-center py-10">
            <p className="text-slate-500 animate-pulse text-lg">Loading secure demos...</p>
          </div>
        )}
        
        <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2">
          {demos.map((demo) => (
            <li
              key={demo.system}
              className="group rounded-2xl border border-slate-200 bg-white p-8 shadow-sm transition-all duration-300 hover:shadow-xl hover:-translate-y-1 hover:border-[#8A2BE2]/30"
            >
              <div className="flex justify-between items-start">
                <h3 className="text-xl font-bold text-slate-800 group-hover:text-[#4682B4] transition-colors">
                  {demo.system}
                </h3>
                {/* Status Badge */}
                <span className={`px-3 py-1 text-xs font-semibold rounded-full ${
                  demo.status === 'active' 
                    ? 'bg-emerald-100 text-emerald-700' 
                    : 'bg-amber-100 text-amber-700'
                }`}>
                  {demo.status.toUpperCase()}
                </span>
              </div>
              
              <p className="mt-4 text-sm text-slate-500 leading-relaxed">
                Experience our {demo.system.toLowerCase()} solution. Built for scale, secured by design.
              </p>
              
              {/* Gradient Button */}
              <button
                onClick={() => window.open(demo.demo_url, "_blank")}
                className="mt-8 w-full inline-flex justify-center items-center rounded-lg bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] px-5 py-3 text-sm font-semibold text-white shadow-md hover:from-[#3b729e] hover:to-[#7a26c7] focus:outline-none focus:ring-2 focus:ring-[#8A2BE2] focus:ring-offset-2 transition-all"
              >
                Access Demo
                <svg className="ml-2 -mr-1 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" />
                </svg>
              </button>
            </li>
          ))}
        </ul>
      </div>
    </main>
  );
}
