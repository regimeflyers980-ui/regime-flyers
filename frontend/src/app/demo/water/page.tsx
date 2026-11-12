"use client";

import { useEffect, useState } from "react";
import Link from "next/link";

interface MeterReading {
  reading_date: string;
  previous_units: number;
  current_units: number;
  units_used: number;
  source: string;
}

interface TenantData {
  full_name: string;
  house_number: string;
  floor_no: number;
  meter_number: string;
  recent_readings: MeterReading[];
}

interface AdminData {
  total_houses: number;
  active_tenants: number;
  total_units_billed: number;
}

export default function WaterDemo() {
  // 1. Initialize with safe defaults so .map() NEVER crashes
  const [tenantData, setTenantData] = useState<TenantData>({
    full_name: "Loading...",
    house_number: "...",
    floor_no: 0,
    meter_number: "...",
    recent_readings: [],
  });
  
  const [adminData, setAdminData] = useState<AdminData>({
    total_houses: 0,
    active_tenants: 0,
    total_units_billed: 0,
  });

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [tenantRes, adminRes] = await Promise.all([
          fetch("/api/water/tenant"),
          fetch("/api/water/admin")
        ]);

        if (!tenantRes.ok || !adminRes.ok) {
          throw new Error("Failed to fetch data from server. Is the backend running?");
        }

        const tenant = await tenantRes.json();
        const admin = await adminRes.json();
        
        // Ensure recent_readings is an array before setting state
        setTenantData({
          ...tenant,
          recent_readings: Array.isArray(tenant.recent_readings) ? tenant.recent_readings : []
        });
        setAdminData(admin);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-slate-50">
        <p className="text-slate-500 text-lg animate-pulse">Loading System Data...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-slate-50 p-4">
        <div className="bg-white p-8 rounded-2xl shadow-xl max-w-md text-center border-t-4 border-red-500">
          <h2 className="text-2xl font-bold text-slate-800 mb-4">System Offline</h2>
          <p className="text-slate-600 mb-6">{error}</p>
          <button onClick={() => window.location.reload()} className="bg-[#4682B4] text-white px-6 py-2 rounded-lg font-semibold hover:bg-[#3b729e]">
            Retry Connection
          </button>
          <Link href="/" className="block mt-4 text-sm text-slate-500 hover:text-[#8A2BE2] underline">
            Return to Home
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-slate-100 p-4 sm:p-8 font-sans">
      <div className="max-w-7xl mx-auto mb-8 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h1 className="text-3xl font-extrabold text-slate-800">Water Management System</h1>
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
          </div>
          <div className="p-6">
            <div className="grid grid-cols-3 gap-4 mb-8">
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Total Houses</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData.total_houses}</p>
              </div>
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Active Tenants</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData.active_tenants}</p>
              </div>
              <div className="bg-slate-50 p-4 rounded-xl border border-slate-100 text-center">
                <p className="text-sm text-slate-500">Units Billed</p>
                <p className="text-2xl font-bold text-[#4682B4]">{adminData.total_units_billed}</p>
              </div>
            </div>
          </div>
        </div>

        {/* TENANT PORTAL */}
        <div className="bg-white rounded-2xl shadow-sm border border-slate-200 overflow-hidden">
          <div className="bg-slate-800 px-6 py-4">
            <h2 className="text-xl font-bold text-white">Tenant Portal View</h2>
            <p className="text-slate-300 text-sm">Logged in as: {tenantData.full_name}</p>
          </div>
          <div className="p-6">
            <div className="mb-6 flex justify-between items-center bg-slate-50 p-4 rounded-xl border border-slate-100">
              <div>
                <p className="text-sm text-slate-500">House / Floor</p>
                <p className="font-semibold text-slate-800">{tenantData.house_number} (Floor {tenantData.floor_no})</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-slate-500">Meter Number</p>
                <p className="font-semibold text-slate-800">{tenantData.meter_number}</p>
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
                  {tenantData.recent_readings.length === 0 ? (
                    <tr>
                      <td colSpan={5} className="text-center py-4 text-slate-500">No readings found.</td>
                    </tr>
                  ) : (
                    tenantData.recent_readings.map((reading, i) => (
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
                    ))
                  )}
                </tbody>
              </table>
            </div>
          </div>
        </div>

      </div>
    </div>
  );
}
