import { create } from 'zustand'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useEffect, useId, useState } from 'react'
import { persist } from 'zustand/middleware'
import { panic } from '@/lib/util'

/**
 * React hook for accessing the address of game server.
 *
 * To use outside of React components, use `.getState()`.
 */
export const useHostStore = create<{
  /**
   * Address of the game server.
   */
  host?: string

  /**
   * Update the address of the game server.
   *
   * @param host new game server
   */
  setHost: (host: string) => void

  /**
   * Whether the state has been loaded from the storage.
   */
  loaded: boolean

  /**
   * Set the loading status.
   *
   * @param loaded whether the state has been loaded
   */
  setLoaded: (loaded: boolean) => void
}>()(
  persist(
    (set) => ({
      setHost: (host) => set({ host }),
      loaded: false,
      setLoaded: (loaded) => set({ loaded }),
    }),
    {
      name: 'host',
      onRehydrateStorage:
        ({ setLoaded }) =>
        () =>
          setLoaded(true),
    }
  )
)

/**
 * An editable display for the address of the game server.
 *
 * Would automatically pop up if the address is not set.
 */
export const Host = () => {
  const { host, setHost, loaded } = useHostStore()
  const [open, setOpen] = useState(false)
  const formId = useId()
  const inputId = useId()

  useEffect(() => {
    if (!loaded) return
    if (host == undefined) setOpen(true)
  }, [loaded, host])

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button variant="link">{host}</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Connect to server</DialogTitle>
          <DialogDescription>
            Enter hostname of the game server to connect to.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <form
            id={formId}
            className="grid grid-cols-4 items-center gap-4"
            onSubmit={(e) => {
              e.preventDefault()
              const form = new FormData(e.currentTarget)
              const host = form.get('host')?.toString() ?? panic()
              setOpen(false)
              setHost(host)
            }}
          >
            <Label htmlFor={inputId} className="text-right">
              Server
            </Label>
            <Input
              id={inputId}
              name="host"
              placeholder="host:port"
              className="col-span-3"
              required
            />
          </form>
        </div>
        <DialogFooter>
          <Button form={formId} type="submit" className="w-full">
            Save
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
