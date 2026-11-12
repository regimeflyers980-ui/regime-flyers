"use client";

import { useEffect, useState } from "react";
import Link from "next/link";

export default function WaterDemo() {
  const [tenantData, setTenantData] = useState<any>(null);
  const [adminData, setAdminData] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      fetch("/api/water/tenant").then((res) => res.json()),
      fetch("/api/water/admin").then((res) => res.json()),
    ])
      .then(([tenant, admin]) => {
        setTenantData(tenant);
        setAdminData(admin);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to fetch data", err);
        setLoading(false);
      });
  }, []);

  if (loading) return <div className="min-h-screen flex items-center justify-center text-slate-500">Loading System Data...</div>;

  return (
    <div className="min-h-screen bg-slate-100 p-4 sm:p-8">
      {/* Header */}
      <div className="max-w-7xl mx-auto mb-8 flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-slate-800">Water Management System</h1>
          <p className="text-slate-500">Interactive Demo Environment</p>
        </div>
        <Link href="/" className="text-[#4682B4] hover:text-[#8A2BE2] font-semibold underline transition-colors">
          &larr; Back to Home
        </Link>
      </div>

      <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-8">
        
        {/* ADMIN DASHBOARD */}
        <div className="bg-white rounded-2xl shadow-sm border border-slate-200 overflow-hidden">
          <div className="bg-gradient-to-r from-[#4682B4] to-[#8A2BE2] px-6 py-4">
            <h2 className="text-xl font-bold text-white">Admin Dashboard View</h2>
            <p className="text-blue-100 text-sm">Regime Flyers Staff Overview</p>
          </div>
          <div className="p-6">
            {/* Stats Row */}
            <div className="grid grid-cols-3 gap-4 mb-8">
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Total Houses</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData?.total_houses}</p>
              </div>
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Active Tenants</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData?.active_tenants}</p>
              </div>
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Units Billed</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData?.total_units_billed}</p>
              </div>
            </div>
            
            <h3 className="font-semibold text-slate-800 mb-4">Recent OCR Scans Pending Approval</h3>
            <ul className="space-y-3">
              {adminData?.recent_scans.map((scan: string, i: number) => (
                <li key={i} className="flex justify-between items-center p-3 bg-slate-50 rounded-lg border border-slate-100 text-sm">
                  <span className="font-medium text-slate-700">{scan}</span>
                  <button className="px-3 py-1 bg-[#8A2BE2]/10 text-[#8A2BE2] rounded-md font-semibold hover:bg-[#8A2BE2]/20 transition">Verify</button>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {/* TENANT PORTAL */}
        <div className="bg-white rounded-2xl shadow-sm border border-slate-200 overflow-hidden">
          <div className="bg-slate-800 px-6 py-4">
            <h2 className="text-xl font-bold text-white">Tenant Portal View</h2>
            <p className="text-slate-300 text-sm">Logged in as: {tenantData?.full_name}</p>
          </div>
          <div className="p-6">
            <div className="mb-6 flex justify-between items-center bg-slate-50 p-4 rounded-xl border border-slate-100">
              <div>
                <p className="text-sm text-slate-500">House / Floor</p>
                <p className="font-semibold text-slate-800">{tenantData?.house_number} (Floor {tenantData?.floor_no})</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-slate-500">Meter Number</p>
                <p className="font-semibold text-slate-800">{tenantData?.meter_number}</p>
              </div>
            </div>

            <h3 className="font-semibold text-slate-800 mb-4">Usage History</h3>
            <div className="overflow-x-auto">
              <table className="w-full text-sm text-left">
                <thead className="text-xs text-slate-500 uppercase bg-slate-50">
                  <tr>
                    <th className="px-4 py-3">Date</th>
                    <th className="px-4 py-3">Previous</th>
                    <th className="px-4 py-3">Current</th>
                    <th className="px-4 py-3">Used</th>
                    <th className="px-4 py-3">Source</th>
                  </tr>
                </thead>
                <tbody>
                  {tenantData?.recent_readings.map((reading: any, i: number) => (
                    <tr key={i} className="border-b border-slate-100 last:border-0 hover:bg-slate-50 transition">
                      <td className="px-4 py-3 font-medium text-slate-900">{reading.reading_date}</td>
                      <td className="px-4 py-3 text-slate-600">{reading.previous_units}</td>
                      <td className="px-4 py-3 text-slate-600">{reading.current_units}</td>
                      <td className="px-4 py-3 font-bold text-[#4682B4]">{reading.units_used}</td>
                      <td className="px-4 py-3">
                        <span className={`px-2 py-1 text-xs rounded-full ${reading.source === 'OCR_SCAN' ? 'bg-[#8A2BE2]/10 text-[#8A2BE2]' : 'bg-slate-200 text-slate-600'}`}>
                          {reading.source}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
}
