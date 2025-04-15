'use client'

import { create } from 'zustand'
import { persist } from 'zustand/middleware' // Import persist middleware

export const username = create<{
  name?: string
  setName: (name: string) => void
}>()(
  persist(
    // Wrap the initializer with persist
    (set) => ({
      setName: (username) => set(() => ({ name: username })),
    }),
    {
      name: 'username-storage', // Unique name for localStorage key
    }
  )
)

export const Username = () => {
  const name = username((state) => state.name)
  return <>{name}</> // Example: render the name
}

// Optional: A hook might be a more conventional way to access the state
export const useUsername = () => {
  return username((state) => state.name)
}

// Optional: A hook to access the setter function
export const useSetUsername = () => {
  return username((state) => state.setName)
}
