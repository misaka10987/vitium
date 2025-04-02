"use client"
import { useState } from 'react';
import { add } from 'vitium-api';
import { popup } from './popup';
import { login } from './login';
import { GamePage } from './gamepage';
import IconSvg from './icon.svg'; // Import the SVG

export default function Page() {
  const [LoginState, setLoginState] = useState(false);
  const [credentials, setCredentials] = useState({ username: '', password: '' });

  if (!LoginState) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white p-4 relative">
        {/* <div className="absolute left-16 h-full flex items-center overflow-visible pointer-events-none opacity-20">
          <img 
            src={IconSvg.src || IconSvg} 
            alt="" 
            className="w-[50vw] h-auto -ml-[10vw]" 
            style={{ transform: 'scaleY(2.2) scaleX(1.6)' }}
          />
        </div> */}

        <img
          src={IconSvg.src || IconSvg}
          alt="Vitium Logo"
          className="relative flex items-center pointer-events-none opacity-80 w-[12vw] h-auto"
        />

        <h1 className="text-4xl font-bold mb-8 z-10">Welcome to Vitium</h1>
        <p className="text-base mb-4 text-gray-400 z-10">Version - Dev rolling</p>
        <div className="py-3"></div>
        <button
          onClick={async () => {
            try {
              const loginResult = await login();
              if (loginResult) {
                setCredentials(loginResult);
                console.log("Login successful:");
                setLoginState(true);
              }
              else {
                console.log("Login cancelled or failed.");
              }
            } catch (error) {
              console.error("Unexpected error during login", error);
            }
          }}
          className="px-6 py-2 bg-purple-600 hover:bg-purple-300 text-white font-semibold rounded-md transition duration-200 ease-in-out z-10">
          Login
        </button>
      </div>
    ) // show the login page when not logged in
  }

  return (
    <GamePage />
  )
}