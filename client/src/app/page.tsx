"use client"
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { GamePage } from './gamepage/page';
import IconSvg from './icon.svg'; // Import the SVG

export default function Page() {
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    // Check if user is logged in
    const checkLoginState = () => {
      // Here you would typically check for authentication token
      // For now, we're assuming not logged in
      const isAuthenticated = false;
      setIsLoading(false);
      
      if (!isAuthenticated) {
        router.push('/login');
      }
    };
    
    checkLoginState();
  }, [router]);

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
        <img
          src={IconSvg.src || IconSvg}
          alt="Vitium Logo"
          className="relative flex items-center pointer-events-none opacity-80 w-[12vw] h-auto animate-pulse"
        />
        <p className="mt-4">Loading...</p>
      </div>
    );
  }

  return <GamePage />;
}