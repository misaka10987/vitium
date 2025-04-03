"use client"
import { useState, useEffect } from 'react'; // Add useEffect import
import { useRouter } from 'next/navigation';
import IconSvg from '../icon.svg';
import { login } from './login';

export default function LoginPage() {
    const router = useRouter();
    const [isLoggingIn, setIsLoggingIn] = useState(false);

    const handleLoginClick = async () => {
        setIsLoggingIn(true);
        try {
            const loginResult = await login();
            if (loginResult) {
                console.log("Login successful:");
                // Set the login status in sessionStorage
                sessionStorage.setItem('isLoggedIn', 'true');
                // Redirect to main page after successful login
                setIsLoggingIn(false);
                router.push('/');
            } else {
                console.log("Login cancelled or failed.");
                setIsLoggingIn(false);
            }
        } catch (error) {
            console.error("Unexpected error during login", error);
            setIsLoggingIn(false);
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white p-4 relative">
            <img
                src={IconSvg.src || IconSvg}
                alt="Vitium Logo"
                className="relative flex items-center pointer-events-none opacity-80 w-[12vw] h-auto"
            />

            <h1 className="text-4xl font-bold mb-8 z-10">Welcome to Vitium</h1>
            <p className="text-base mb-4 text-gray-400 z-10">Version - Dev rolling</p>
            <div className="py-3"></div>
            <button
                onClick={handleLoginClick}
                disabled={isLoggingIn}
                className="px-6 py-2 bg-purple-600 hover:bg-purple-300 text-white font-semibold rounded-md transition duration-200 ease-in-out z-10 disabled:bg-purple-800 disabled:opacity-70"
            >
                {isLoggingIn ? 'Logging in...' : 'Login'}
            </button>
        </div>
    );
}
