"use client"

import { useEffect } from 'react'
import { useCloudStore } from '../stores/cloud/useCloudStore'
import DriveTree from './DriveTree'

export default function GoogleDrive () {
  const { setGoogleDrives, googleDrives, loading } = useCloudStore()
  useEffect(() => {
    setGoogleDrives()
  },[])
  if (loading) return <div>Loading drives...</div>;
  
  if(!googleDrives || googleDrives?.length < 1) {
    return <div>
      No google drives fetched
    </div>
  }
    return (
      <div className="flex gap-4 bg-gray-100 p-4 h-screen">
        <div className="w-80 bg-white p-2 rounded shadow">
          <h2 className="font-bold mb-2">Google Drives</h2>
          {googleDrives.map((drive, i) => (
            <DriveTree key={i} drive={drive} />
          ))}
        </div>
  
        {/* Right pane could show file preview later */}
        <div className="flex-1 bg-white rounded shadow p-4">
          <h2>Select a file...</h2>
        </div>
      </div>
    );
  
}