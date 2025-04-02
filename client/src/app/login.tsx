import React, { useState, FC, FormEvent, MouseEvent } from 'react';
import ReactDOM from 'react-dom/client'; // Use client for React 18+

// Define the props for the internal LoginPopup component
interface LoginPopupProps {
  onLogin: (credentials: { username: string; password: string }) => void;
  onCancel: () => void;
  errorMessage?: string | null; // Optional error message prop
}

// The internal React component for the popup UI
const LoginPopup: FC<LoginPopupProps> = ({ onLogin, onCancel, errorMessage }) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault(); // Prevent default form submission
    // Basic validation (optional, add more as needed)
    if (!username || !password) {
        // You could pass an error handling function instead of just logging
        console.error("Username and password are required.");
        // Or update an internal error state if managing errors within the popup
        return;
    }
    onLogin({ username, password });
  };

  // Handle clicks outside the modal content to close it
  const handleOverlayClick = (event: MouseEvent<HTMLDivElement>) => {
    // Check if the click target is the overlay itself, not its children
    if (event.target === event.currentTarget) {
      onCancel();
    }
  };

  return (
    // Overlay container
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-60 backdrop-blur-sm"
      onClick={handleOverlayClick} // Add click handler to overlay
    >
      {/* Modal content container */}
      <div className="bg-gray-800 text-white rounded-lg shadow-xl p-6 w-full max-w-sm transform transition-all scale-100 opacity-100">
        {/* Prevent clicks inside the modal from bubbling up to the overlay */}
        <div onClick={(e) => e.stopPropagation()}>
          <h2 className="text-2xl font-semibold mb-6 text-center text-purple-400">Login</h2>
          <form onSubmit={handleSubmit}>
            {/* Username Field */}
            <div className="mb-4">
              <label
                htmlFor="username"
                className="block text-sm font-medium text-gray-300 mb-1"
              >
                Username
              </label>
              <input
                type="text"
                id="username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                required
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition duration-150 ease-in-out"
                placeholder="Enter your username"
                autoComplete="username" // Helps with password managers
              />
            </div>

            {/* Password Field */}
            <div className="mb-6">
              <label
                htmlFor="password"
                className="block text-sm font-medium text-gray-300 mb-1"
              >
                Password
              </label>
              <input
                type="password"
                id="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent transition duration-150 ease-in-out"
                placeholder="Enter your password"
                autoComplete="current-password" // Helps with password managers
              />
            </div>

             {/* Error Message Display */}
            {errorMessage && (
                <p className="text-red-500 text-sm mb-4 text-center">{errorMessage}</p>
            )}

            {/* Action Buttons */}
            <div className="flex items-center justify-between">
              <button
                type="button" // Important: type="button" to prevent form submission
                onClick={onCancel}
                className="px-4 py-2 rounded-md text-gray-300 bg-gray-600 hover:bg-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-gray-500 transition duration-150 ease-in-out"
              >
                Cancel
              </button>
              <button
                type="submit"
                className="px-4 py-2 rounded-md text-white bg-purple-600 hover:bg-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-purple-500 font-semibold transition duration-150 ease-in-out"
              >
                Login
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};

// --- Public API ---

// This is the function you will call to show the login popup
export const login = (initialErrorMessage?: string | null): Promise<{ username: string; password: string } | null> => {
  return new Promise((resolve) => {
    // Create a temporary div to mount the React component
    const mountPoint = document.createElement('div');
    mountPoint.id = 'login-popup-mount-point'; // Optional: for easier debugging
    document.body.appendChild(mountPoint);

    // Use createRoot for React 18+
    const root = ReactDOM.createRoot(mountPoint);

    let currentErrorMessage = initialErrorMessage;

    // Function to clean up: unmount React component and remove the div
    const cleanup = () => {
      root.unmount(); // Unmount the component
      document.body.removeChild(mountPoint); // Remove the div from DOM
    };

    // Function to handle successful login
    const handleLogin = (credentials: { username: string; password: string }) => {
      cleanup();
      resolve(credentials); // Resolve the promise with credentials
    };

    // Function to handle cancellation
    const handleCancel = () => {
      cleanup();
      resolve(null); // Resolve the promise with null for cancellation
    };

    // Function to update the error message if needed (e.g., after a failed API call)
    // This is slightly more advanced, showing how the promise *could* interact
    // But for simplicity, we'll mainly pass the initial error message.
    // To update dynamically, the login function would need more logic.
    const renderPopup = (errorMsg: string | null | undefined) => {
         root.render(
            <React.StrictMode> {/* Good practice */}
                <LoginPopup
                    onLogin={handleLogin}
                    onCancel={handleCancel}
                    errorMessage={errorMsg}
                />
            </React.StrictMode>
        );
    }

    // Initial render
    renderPopup(currentErrorMessage);

  });
};

// Default export can be useful if you import the file directly sometimes,
// but the main API is the named export `login`.
// export default LoginPopup; // Usually you wouldn't export the component itself