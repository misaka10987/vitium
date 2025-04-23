'use client'

import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Host } from '@/components/host'
import { useUserStore } from '@/components/user'
import { useId, useState } from 'react'
import Link from 'next/link'
import { grabToken } from '@/lib/auth'
import { useRouter } from 'next/navigation'

export function LoginForm({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  const userInputId = useId()
  const passInputId = useId()
  const [wrongCredentials, setWrongCredentials] = useState(false)
  const { setUsername } = useUserStore()
  const router = useRouter()
  return (
    <div className={cn('flex flex-col gap-6', className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle>
            Login to: <Host />
          </CardTitle>
          <CardDescription>
            Login to the above game server with your username and password
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={async (e) => {
              e.preventDefault()
              const data = new FormData(e.currentTarget)
              const user = data.get('user')?.toString()
              const pass = data.get('pass')?.toString()
              if (user == undefined || pass == undefined)
                throw new Error('impossible')
              const res = await grabToken(user, pass)
              if (!res.ok) {
                setWrongCredentials(true)
                return
              }
              setUsername(user)
              router.replace('/game')
            }}
          >
            <div className="flex flex-col gap-6">
              <div className="grid gap-3">
                <Label htmlFor={userInputId}>Username</Label>
                <Input
                  id={userInputId}
                  name="user"
                  placeholder="username"
                  required
                />
              </div>
              <div className="grid gap-3">
                <div className="flex items-center">
                  <Label htmlFor={passInputId}>Password</Label>
                  <Link
                    href="mailto:mail@example.com"
                    className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                  >
                    Forgot your password?
                  </Link>
                </div>
                <Input id={passInputId} name="pass" type="password" required />
              </div>
              <div className="flex h-0 items-center">
                {wrongCredentials && (
                  <p className="text-sm text-red-600">
                    Wrong username or password
                  </p>
                )}
              </div>
              <div className="flex flex-col gap-3">
                <Button type="submit" className="w-full">
                  Login
                </Button>
              </div>
            </div>
            <div className="mt-4 text-center text-sm">
              Don&apos;t have an account?{' '}
              <Link href="/signup" className="underline underline-offset-4">
                Sign up
              </Link>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
