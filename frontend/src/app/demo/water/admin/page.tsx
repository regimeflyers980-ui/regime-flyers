"use client";

import { useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";

export default function AdminEntry() {
  const router = useRouter();
  const [previousUnits, setPreviousUnits] = useState("");
  const [currentUnits, setCurrentUnits] = useState("");
  const [source, setSource] = useState("MANUAL");
  const [isScanning, setIsScanning] = useState(false);
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  // Simulate OCR Extraction
  const handleSimulateOCR = () => {
    setIsScanning(true);
    setMessage(null);
    setTimeout(() => {
      setPreviousUnits("145"); // Mock previous
      setCurrentUnits("168");  // Mock extracted OCR number
      setSource("OCR_SCAN");
      setIsScanning(false);
      setMessage({ type: "success", text: "OCR Complete! Please verify the extracted numbers before saving." });
    }, 2000);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setMessage(null);

    try {
      const res = await fetch("/api/water/reading", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          meter_id: 1, // Hardcoded for demo (WM-99821)
          previous_units: parseInt(previousUnits),
          current_units: parseInt(currentUnits),
          source: source,
        }),
      });

      if (res.ok) {
        setMessage({ type: "success", text: "Reading saved successfully! Returning to dashboard..." });
        setTimeout(() => router.push("/demo/water"), 2000);
      } else {
        setMessage({ type: "error", text: "Failed to save reading." });
      }
    } catch (err) {
      setMessage({ type: "error", text: "Network error." });
    }
  };

  return (
    <div className="min-h-screen bg-slate-100 p-4 sm:p-8 font-sans flex justify-center items-center">
      <div className="bg-white rounded-2xl shadow-xl w-full max-w-lg overflow-hidden border border-slate-200">
        
        <div className="bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] px-6 py-4 flex justify-between items-center">
          <div>
            <h2 className="text-xl font-bold text-white">Enter Water Reading</h2>
            <p className="text-blue-100 text-sm">Meter: WM-99821 (John Doe)</p>
          </div>
          <Link href="/demo/water" className="text-white hover:text-blue-100 text-sm underline">Cancel</Link>
        </div>

        <div className="p-6">
          {message && (
            <div className={`p-4 mb-6 rounded-lg text-sm ${message.type === "success" ? "bg-green-50 text-green-700" : "bg-red-50 text-red-700"}`}>
              {message.text}
            </div>
          )}

          <div className="mb-6 p-4 bg-slate-50 border border-slate-200 rounded-xl border-dashed text-center">
            <p className="text-slate-500 text-sm mb-3">Have a photo of the meter?</p>
            <button 
              type="button" 
              onClick={handleSimulateOCR}
              disabled={isScanning}
              className="bg-white border border-[#8A2BE2] text-[#8A2BE2] px-4 py-2 rounded-lg font-semibold hover:bg-[#8A2BE2]/5 transition disabled:opacity-50"
            >
              {isScanning ? "Scanning Image..." : "Upload & Scan Image (OCR)"}
            </button>
          </div>

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-slate-700 mb-1">Previous Units</label>
              <input 
                type="number" 
                value={previousUnits}
                onChange={(e) => setPreviousUnits(e.target.value)}
                className="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-[#4682B4] outline-none"
                required
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 mb-1">Current Units</label>
              <input 
                type="number" 
                value={currentUnits}
                onChange={(e) => setCurrentUnits(e.target.value)}
                className="w-full px-4 py-2 border border-slate-300 rounded-lg focus:ring-2 focus:ring-[#4682B4] outline-none"
                required
              />
            </div>
            
            <button 
              type="submit" 
              className="w-full mt-6 bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] text-white py-3 rounded-lg font-bold hover:from-[#3b729e] hover:to-[#7a26c7] transition-all shadow-md"
            >
              Confirm & Save Reading
            </button>
          </form>
        </div>
      </div>
    </div>
  );
}
