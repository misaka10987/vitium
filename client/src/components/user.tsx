'use client'

import { create } from 'zustand'
import { persist } from 'zustand/middleware' // Import persist middleware

export const useUserStore = create<{
  username?: string
  setUsername: (name: string) => void
}>()(
  persist(
    (set) => ({
      setUsername: (username) => set(() => ({ username: username })),
    }),
    {
      name: 'username',
    }
  )
)

export const userStore = useUserStore

export const User = () => {
  const { username } = useUserStore()
  return username
}
