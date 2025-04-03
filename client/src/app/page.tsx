"use client"
import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import IconSvg from './icon.svg'; // Import the SVG

export default function Page() {
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    // Only set 'isLoggedIn' to 'false' if it doesn't exist
    if (sessionStorage.getItem('isLoggedIn') === null) {
      sessionStorage.setItem('isLoggedIn', 'false');
    }
    
    const loggedInStatus = sessionStorage.getItem('isLoggedIn') === 'true';
    
    // Check if user is logged in
    const checkLoginState = () => {
      // Here you would typically check for authentication token
      setIsLoading(false);
      if (!loggedInStatus) {
        router.push('/login');
      }
      else {
        router.push('/gamepage');
      }
    };
    checkLoginState();
  }, [router]);// prepare stuff, after this the 'loading' would disappear

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
}