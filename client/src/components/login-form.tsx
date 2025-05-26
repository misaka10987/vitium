'use client'

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
import { Host, useHostStore } from '@/components/host'
import { useUserStore } from '@/components/user'
import { useId, useState } from 'react'
import Link from 'next/link'
import { login } from '@/lib/auth'
import { useRouter } from 'next/navigation'
import { panic } from '@/lib/util'
import { match } from 'ts-pattern'

/**
 * User interface for logging in to a certain game server.
 */
export const LoginForm = () => {
  const userInputId = useId()
  const passInputId = useId()
  const [res, setRes] = useState<Response | null>(null)
  const { setUser } = useUserStore()
  const router = useRouter()
  const { host } = useHostStore()
  return (
    <div className="flex flex-col gap-6 select-none">
      <Card>
        <CardHeader>
          <CardTitle>
            Connect to: <Host />
          </CardTitle>
          <CardDescription>
            Connect to the above game server with your username and password
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={async (e) => {
              e.preventDefault()
              const data = new FormData(e.currentTarget)
              const user = data.get('user')?.toString() ?? panic()
              const pass = data.get('pass')?.toString() ?? panic()
              const res = await login(user, pass)
              if (res.ok) {
                setUser(user)
                router.replace('/game')
              }
              setRes(res)
            }}
          >
            <div className="flex flex-col gap-6">
              <div className="grid gap-3">
                <Label htmlFor={userInputId}>Username</Label>
                <Input
                  id={userInputId}
                  name="user"
                  required
                />
              </div>
              <div className="grid gap-3">
                <div className="flex">
                  <Label htmlFor={passInputId}>Password</Label>
                  <Link
                    href={`https://${host}/api/contact`}
                    className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                  >
                    Forgot your password?
                  </Link>
                </div>
                <Input id={passInputId} name="pass" type="password" required />
              </div>
              {
                res != null && !res.ok && <div className="flex">
                  <p className="text-sm text-destructive">
                    {
                      match(res.status)
                        .with(401, () => "False username or password")
                        .otherwise(() => `${res.status} ${res.statusText}`)
                    }
                  </p>
                </div>
              }
              <div className="flex flex-col gap-3">
                <Button type="submit" className="w-full">
                  Login
                </Button>
              </div>
            </div>
            <div className="mt-4 text-center text-sm">
              Don&apos;t have an account?&nbsp;
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
