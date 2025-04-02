"use client"
import { useState } from 'react';
import { add } from 'vitium-api';
import { gameview } from './gameview';

export default function Home() {
  const [isPopupVisible, setPopupVisible] = useState(false);
  const [isLoginFailedVisible, setLoginFailedVisible] = useState(false); // State for login failed popup
  const [username, setUsername] = useState(''); // State for username input
  const [password, setPassword] = useState(''); // State for password input
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  // If logged in, render game view
  if (isLoggedIn) {
    return gameview();
  }

  const openPopup = () => {
    setPopupVisible(true);
  };

  const closePopup = () => {
    setPopupVisible(false);
  };

  const openLoginFailedPopup = () => {
    setLoginFailedVisible(true);
  };

  const closeLoginFailedPopup = () => {
    setLoginFailedVisible(false);
  };

  function login() {
    if (username === "admin" && password === "passwd") {
      setIsLoggedIn(true); // Set logged in state to true
      return null;
    } else {
      openLoginFailedPopup(); // Show login failed popup
      return null;
    }
  } // test mode now

  // page data
  return (
    <div className="welcome-page flex flex-col items-center justify-center min-h-screen bg-black p-4">
      <h1 className="text-4xl font-bold mb-8 text-center">
        Welcome to Vitium
      </h1>
      <p style={{ color: 'gray' }}>
        Version - Development-rolling
      </p>
      <div style={{ height: '30px' }}></div>
      <button
        className="bg-white rounded-lg px-4 py-2"
        onClick={openPopup}
        style={{ color: "black" }}
      >
        Player Login
      </button>

      {isPopupVisible && (
        <div className="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 flex items-center justify-center">
          <div className="bg-black p-8 rounded-lg">
            <h2 className="text-2xl font-bold mb-4">Login</h2>
            {/* Additional login form or content here */}
            <p>Enter your username and password:</p>
            <div className='py-1'></div>
            <input
              type="text"
              placeholder="Username"
              className="border rounded-md p-2 mb-2"
              value={username} // Bind the input value to the state
              onChange={(e) => setUsername(e.target.value)} // Update state on change
            />
            <div></div>
            <input
              type="password"
              placeholder="Password"
              className="border rounded-md p-2 mb-2"
              value={password} // Bind the input value to the state
              onChange={(e) => setPassword(e.target.value)} // Update state on change
            />
            <div className="flex justify-end">
              <button className="bg-gray-500 rounded-lg px-4 py-2 mr-2" onClick={closePopup}>
                Cancel
              </button>
              <button className="bg-purple-500 text-white rounded-lg px-4 py-2" onClick={login}>
                Login
              </button>
            </div>
          </div>
        </div>
      )}

      {isLoginFailedVisible && (
        <div className="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 flex items-center justify-center">
          <div className="bg-black p-8 rounded-lg">
            <h2 className="text-2xl font-bold mb-4">Login Failed</h2>
            <p>Invalid username or password. Please try again.</p>
            <div className='py-2'></div>
            <div className="flex justify-end">
              <button className="bg-red-500 text-white rounded-lg px-4 py-2" onClick={closeLoginFailedPopup}>
                Retry
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
