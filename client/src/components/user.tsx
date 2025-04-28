'use client'

import { create } from 'zustand'
import { persist } from 'zustand/middleware' // Import persist middleware

export const useUserStore = create<{
  user?: string
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

export const userStore = useUserStore

export const User = () => {
  const { user } = useUserStore()
  return user
}
