'use client'

import { Icon } from '@/components/icon'
import { Button } from '@/components/ui/button'
import Link from 'next/link'
import { add, test } from 'vitium-api'

export default function Page() {
  console.debug('WASM initialized: 2+3=', add(2, 3))
  const res = test()
  console.debug('BigInt from WASM test:', res)
  return (
    <div className="flex h-full w-full align-top justify-center">
      <div className="flex flex-col gap-8 align-middle m-4">
        <div className="flex flex-col">
          <Icon width={320} height={320} />
          <article>
            <h1 className="text-4xl font-bold text-center mb-4">Vitium</h1>
            <p className="text-center">The TRPG Framework</p>
          </article>
        </div>
        <div className="flex justify-center">
          <Button asChild variant="secondary" className="m-4 select-none">
            <Link href="/login">Player Login</Link>
          </Button>
        </div>
      </div>
    </div>
  )
}
