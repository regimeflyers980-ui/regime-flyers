"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const router = useRouter();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    
    try {
      const res = await fetch("/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });

      if (res.ok) {
        router.push("/demo/water"); // Redirect on success
      } else {
        setError("Invalid email or password");
      }
    } catch (err) {
      setError("Network error. Is the backend running?");
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-slate-100 p-4">
      <div className="bg-white p-8 rounded-2xl shadow-xl w-full max-w-md border-t-8 border-[#4682B4]">
        <h2 className="text-3xl font-extrabold text-center text-slate-800 mb-2">Welcome Back</h2>
        <p className="text-center text-slate-500 mb-8">Sign in to Regime Flyers</p>
        
        {error && <div className="bg-red-50 text-red-600 p-3 rounded-lg mb-4 text-sm text-center">{error}</div>}

        <form onSubmit={handleLogin} className="space-y-5">
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1">Email</label>
            <input 
              type="email" 
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-[#8A2BE2] focus:border-transparent outline-none transition"
              placeholder="admin@regimeflyers.com"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1">Password</label>
            <input 
              type="password" 
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-[#8A2BE2] focus:border-transparent outline-none transition"
              placeholder="password123"
              required
            />
          </div>
          <button 
            type="submit" 
            className="w-full bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] text-white py-3 rounded-lg font-bold hover:from-[#3b729e] hover:to-[#7a26c7] transition-all shadow-md"
          >
            Sign In
          </button>
        </form>
        <p className="text-center text-sm text-slate-500 mt-6">
          <Link href="/" className="hover:text-[#8A2BE2] underline">Return to Home</Link>
        </p>
      </div>
    </div>
  );
}
