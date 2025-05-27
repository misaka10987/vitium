'use client'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useHostStore } from '@/components/host'
import { useCallback, useRef } from 'react'
import { validateEquals } from 'typia'
import { useForm } from 'react-hook-form'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from './ui/form'

interface Data {
  user: string
  pass: string
  repeatPass: string
}

/**
 * User interface for signing up to a certain game server.
 */
export const SignupForm = () => {
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
    if (valid.data.pass != valid.data.repeatPass)
      return {
        values: {},
        errors: {
          repeatPass: {
            type: 'mismatch',
            message: 'Password mismatch with repeated.',
          },
        },
      }
    return { values: valid.data, errors: {} }
  }, [])

  const form = useForm<Data>({
    resolver,
    defaultValues: {
      user: '',
      pass: '',
      repeatPass: '',
    },
    shouldUnregister: true,
    mode: 'onChange',
  })

  const htmlForm = useRef<HTMLFormElement>(null)

  return (
    <Form {...form}>
      <form
        ref={htmlForm}
        className="flex flex-col gap-6 w-full"
        action={`https://${host}/api/auth`}
        method="POST"
        onSubmit={form.handleSubmit((_) => {
          form.unregister('repeatPass')
          htmlForm.current?.submit()
        })}
      >
        <FormField
          control={form.control}
          name="user"
          render={({ field }) => (
            <FormItem className="w-full">
              <FormLabel>Username</FormLabel>
              <FormControl>
                <Input {...field} autoComplete="username" required />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="pass"
          render={({ field }) => (
            <FormItem className="w-full">
              <FormLabel>Password</FormLabel>
              <FormControl>
                <Input
                  {...field}
                  type="password"
                  autoComplete="new-password"
                  required
                />
              </FormControl>
            </FormItem>
          )}
        />

        <FormField
          control={form.control}
          name="repeatPass"
          render={({ field }) => (
            <FormItem className="w-full">
              <FormLabel>Repeat Password</FormLabel>
              <FormControl>
                <Input
                  {...field}
                  type="password"
                  autoComplete="new-password"
                  required
                />
              </FormControl>
              <FormMessage>
                {form.formState.errors.repeatPass?.message}
              </FormMessage>
            </FormItem>
          )}
        />

        <Button type="submit" className="w-full">
          Sign Up
        </Button>
      </form>
    </Form>
  )
}
