import { create } from 'zustand'

export const username = create<{
  name?: string
  setName: (name: string) => void
}>()((set) => ({
  setName: (username) => set(() => ({ name: username })),
}))

export const Username = () => {
  const name = username((state) => state.name)
  return name
}
