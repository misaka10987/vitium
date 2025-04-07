'use client'

import { LoginForm } from '@/components/login-form'

export default function Page() {
  return (
    <div className="flex w-full h-full justify-center items-center">
      <div className="w-full max-w-sm">
        <LoginForm />
      </div>
    </div>
  )
}
