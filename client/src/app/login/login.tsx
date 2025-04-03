import { Dialog, Transition } from '@headlessui/react'
import { Fragment, useState } from 'react'

export function login(): Promise<boolean> {
  return new Promise((resolve) => {
    // Create a root element for the dialog
    const dialogRoot = document.createElement('div')
    dialogRoot.id = 'login-dialog-root'
    document.body.appendChild(dialogRoot)

    // Render the login dialog
    const LoginDialog = () => {
      const [isOpen, setIsOpen] = useState(true)
      const [username, setUsername] = useState('')
      const [password, setPassword] = useState('')
      const [serverAddress, setServerAddress] = useState('localhost:3000')
      const [error, setError] = useState<string | null>(null)
      const [isLoading, setIsLoading] = useState(false)

      // Function to authenticate with the server
      const authenticateWithServer = async (serverUrl: string, username: string, password: string): Promise<boolean> => {
        try {
          // Ensure URL has the correct protocol
          if (!serverUrl.startsWith('http://') && !serverUrl.startsWith('https://')) {
            serverUrl = `http://${serverUrl}`
          }
          
          // Create Basic Auth header
          const authHeader = 'Basic ' + btoa(`${username}:${password}`)
          
          const response = await fetch(`${serverUrl}/auth`, {
            method: 'GET',
            headers: {
              'Authorization': authHeader,
              'Content-Type': 'application/json'
            }
          })
          
          if (response.ok) {
            // Store authentication info if successful
            localStorage.setItem('authToken', authHeader);
            localStorage.setItem('serverUrl', serverUrl);
            localStorage.setItem('username', username);
          }
          
          return response.ok
        } catch (err) {
          console.error('Authentication error:', err)
          return false
        }
      }

      const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        setIsLoading(true)
        setError(null)
        
        try {
          // Attempt to authenticate with the server
          const success = await authenticateWithServer(serverAddress, username, password)
          
          if (success) {
            setIsOpen(false)
            setTimeout(() => {
              document.body.removeChild(dialogRoot)
              resolve(true)
            }, 300)
          } else {
            setError('Authentication failed. Please check your credentials and server address.')
            setIsLoading(false)
          }
        } catch (err) {
          setError('An error occurred during authentication. Please try again.')
          setIsLoading(false)
        }
      }

      const handleCancel = () => {
        setIsOpen(false)
        setTimeout(() => {
          document.body.removeChild(dialogRoot)
          resolve(false)
        }, 300)
      }

      return (
        <Transition appear show={isOpen} as={Fragment}>
          <Dialog as="div" className="relative z-50" onClose={handleCancel}>
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0"
              enterTo="opacity-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <div className="fixed inset-0 bg-black/60 backdrop-blur-sm" />
            </Transition.Child>

            <div className="fixed inset-0 overflow-y-auto">
              <div className="flex min-h-full items-center justify-center p-4 text-center">
                <Transition.Child
                  as={Fragment}
                  enter="ease-out duration-300"
                  enterFrom="opacity-0 scale-95"
                  enterTo="opacity-100 scale-100"
                  leave="ease-in duration-200"
                  leaveFrom="opacity-100 scale-100"
                  leaveTo="opacity-0 scale-95"
                >
                  <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-gray-800 p-6 text-left align-middle shadow-xl transition-all">
                    <Dialog.Title
                      as="h3"
                      className="text-lg font-medium leading-6 text-purple-300"
                    >
                      Login to your account
                    </Dialog.Title>
                    
                    <form onSubmit={handleSubmit} className="mt-4">
                      <div className="mb-4">
                        <label htmlFor="serverAddress" className="block text-sm font-medium text-gray-300">
                          Server Address
                        </label>
                        <input
                          type="text"
                          id="serverAddress"
                          className="mt-1 block w-full rounded-md bg-gray-700 border-gray-600 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-500 focus:ring-opacity-50 text-white py-2 px-3"
                          value={serverAddress}
                          onChange={(e) => setServerAddress(e.target.value)}
                          required
                        />
                      </div>

                      <div className="mb-4">
                        <label htmlFor="username" className="block text-sm font-medium text-gray-300">
                          Username
                        </label>
                        <input
                          type="text"
                          id="username"
                          className="mt-1 block w-full rounded-md bg-gray-700 border-gray-600 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-500 focus:ring-opacity-50 text-white py-2 px-3"
                          value={username}
                          onChange={(e) => setUsername(e.target.value)}
                          required
                        />
                      </div>

                      <div className="mb-4">
                        <label htmlFor="password" className="block text-sm font-medium text-gray-300">
                          Password
                        </label>
                        <input
                          type="password"
                          id="password"
                          className="mt-1 block w-full rounded-md bg-gray-700 border-gray-600 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-500 focus:ring-opacity-50 text-white py-2 px-3"
                          value={password}
                          onChange={(e) => setPassword(e.target.value)}
                          required
                        />
                      </div>

                      {error && (
                        <div className="mb-4 p-2 bg-red-900/50 border border-red-500 rounded-md">
                          <p className="text-sm text-red-300">{error}</p>
                        </div>
                      )}

                      <div className="mt-6 flex justify-end space-x-3">
                        <button
                          type="button"
                          className="inline-flex justify-center rounded-md border border-transparent bg-gray-600 px-4 py-2 text-sm font-medium text-white hover:bg-gray-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-purple-500 focus-visible:ring-offset-2"
                          onClick={handleCancel}
                          disabled={isLoading}
                        >
                          Cancel
                        </button>
                        <button
                          type="submit"
                          className="inline-flex justify-center rounded-md border border-transparent bg-purple-600 px-4 py-2 text-sm font-medium text-white hover:bg-purple-700 focus:outline-none focus-visible:ring-2 focus-visible:ring-purple-500 focus-visible:ring-offset-2 disabled:bg-purple-500/50"
                          disabled={isLoading}
                        >
                          {isLoading ? 'Logging in...' : 'Login'}
                        </button>
                      </div>
                    </form>
                  </Dialog.Panel>
                </Transition.Child>
              </div>
            </div>
          </Dialog>
        </Transition>
      )
    }

    // Use ReactDOM to render the dialog
    const ReactDOM = require('react-dom/client')
    const root = ReactDOM.createRoot(dialogRoot)
    root.render(<LoginDialog />)
  })
}