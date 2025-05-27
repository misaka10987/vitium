'use client'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useHostStore } from '@/components/host'
import { useUserStore } from '@/components/user'
import { useCallback, useState } from 'react'
import Link from 'next/link'
import { login } from '@/lib/auth'
import { useRouter } from 'next/navigation'
import { match } from 'ts-pattern'
import { useForm } from 'react-hook-form'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from './ui/form'
import { validateEquals } from 'typia'

interface Data {
  user: string
  pass: string
}

/**
 * User interface for logging in to a certain game server.
 */
export const LoginForm = () => {
  const [res, setRes] = useState<Response | null>(null)
  const { setUser } = useUserStore()
  const router = useRouter()
  const { host } = useHostStore()

  const resolver = useCallback((data: unknown) => {
    const valid = validateEquals<Data>(data)
    if (!valid.success)
      return {
        values: {},
        errors: valid.errors.reduce(
          (all, curr) => ({
            ...all,
            [curr.path]: {
              type: 'typia',
              message: `${curr.value}: expected ${curr.expected}`,
            },
          }),
          {}
        ),
      }
    return { values: valid.data, errors: {} }
  }, [])

  const form = useForm<Data>({
    resolver,
    defaultValues: {
      user: '',
      pass: '',
    },
  })

  const submit = async ({ user, pass }: Data) => {
    const res = await login(user, pass)
    if (res.ok) {
      setUser(user)
      router.replace('/game')
    }
    setRes(res)
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(submit)}
        className="flex flex-col gap-6 w-full"
      >
        <FormField
          control={form.control}
          name="user"
          render={({ field }) => (
            <FormItem className="w-full">
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input {...field} required />
              </FormControl>
            </FormItem>
          )}
        ></FormField>
        <FormField
          control={form.control}
          name="pass"
          render={({ field }) => (
            <FormItem className="w-full">
              <FormLabel>
                Password
                <Link
                  href={`https://${host}/api/contact`}
                  className="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                >
                  Forgot your password?
                </Link>
              </FormLabel>
              <FormControl>
                <Input {...field} required />
              </FormControl>
            </FormItem>
          )}
        ></FormField>
        {res != null && !res.ok && (
          <FormMessage>
            {match(res.status)
              .with(401, () => 'False username or password')
              .otherwise(() => `${res.status} ${res.statusText}`)}
          </FormMessage>
        )}
        <Button type="submit" className="w-full">
          Login
        </Button>
      </form>
    </Form>
  )
}
