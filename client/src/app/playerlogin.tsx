import { useState } from 'react';

// Add setUsername to props
export function PlayerLoginPage({ 
  setIsLoggedIn, 
  setUsername 
}: { 
  setIsLoggedIn: (value: boolean) => void;
  setUsername: (value: string) => void;
}) {
  const [isPopupVisible, setPopupVisible] = useState(false);
  const [isLoginFailedVisible, setLoginFailedVisible] = useState(false);
  const [usernameInput, setUsernameInput] = useState(''); // Renamed to avoid confusion
  const [password, setPassword] = useState('');

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
    if (usernameInput === "admin" && password === "passwd") {
      setUsername(usernameInput); // Store username in parent state
      setIsLoggedIn(true);
    } else {
      openLoginFailedPopup();
    }
  }

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
              value={usernameInput} // Bind the input value to the state
              onChange={(e) => setUsernameInput(e.target.value)} // Update state on change
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

// Keep this for backwards compatibility
export const playerlogin_page = PlayerLoginPage;
