import { create } from 'zustand'
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { useId, useState } from 'react'
import Link from 'next/link'

export const useHostStore = create<{
  hostname?: string
  setHostname: (name: string) => void
}>()((set) => ({
  setHostname: (name) => set(() => ({ hostname: name })),
}))

const PromptHost = () => { }

export const Host = () => {
  const { hostname, setHostname } = useHostStore()
  const [open, setOpen] = useState(hostname == undefined)
  const [input, setInput] = useState<string>('')
  const formId = useId()
  const inputId = useId()
  // if (hostname != undefined) return <Button variant='link' onClick={() => setOpen(true)}>{hostname}</Button>
  return <Dialog open={open} onOpenChange={setOpen}>
    <DialogTrigger asChild>
      <Button variant="link">{hostname}</Button>
    </DialogTrigger>
    <DialogContent className="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>Connect to server</DialogTitle>
        <DialogDescription>
          Enter hostname of the game server to connect to.
        </DialogDescription>
      </DialogHeader>
      <div className="grid gap-4 py-4">
        <form id={formId} className="grid grid-cols-4 items-center gap-4"
          onSubmit={(e) => {
            e.preventDefault()
            setOpen(false)
            setHostname(input)
          }}>
          <Label htmlFor={inputId} className="text-right">
            Server
          </Label>
          <Input id={inputId} placeholder="host:port" className="col-span-3" required
            onChange={e => setInput(e.currentTarget.value)} />
        </form>
      </div>
      <DialogFooter>
        <Button form={formId} type='submit' className='w-full'>Save</Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
}
