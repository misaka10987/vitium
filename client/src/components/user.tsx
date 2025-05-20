'use client'

import { create } from 'zustand'
import { persist } from 'zustand/middleware' // Import persist middleware

/**
 * React hook for accessing the username stored.
 */
export const useUserStore = create<{
  /**
   * The username.
   */
  user?: string
  /**
   * Update the username.
   * @param name new username
   */
  setUser: (name: string) => void
}>()(
  persist(
    (set) => ({
      setUser: (user) => set(() => ({ user })),
    }),
    {
      name: 'user',
    }
  )
)

/**
 * A non-hook api for acessing the username outside of React components.
 *
 * Use `.getState()` for visiting the username.
 */
export const userStore = useUserStore

/**
 * A component that displays the current username (or empty if undefined).
 */
export const User = () => {
  const { user } = useUserStore()
  return user
}
