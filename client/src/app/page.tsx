'use client'

import { Icon } from '@/components/icon'
import { Button } from '@/components/ui/button'
import Link from 'next/link'
import { assert } from 'typia'
import { test } from 'vitium-api'

export default function Page() {
  const big = test()
  assert<bigint>(big)
  console.assert(big == BigInt(42))
  return (
    <div className="flex h-full w-full align-top justify-center">
      <div className="flex flex-col gap-8 align-middle m-4">
        <div className="flex flex-col">
          <Icon width={320} height={320} priority />
          <article>
            <h1 className="text-4xl font-bold text-center mb-4">Vitium</h1>
            <p className="text-center">The TRPG Framework</p>
          </article>
        </div>
        <div className="flex justify-center">
          <Button asChild variant="secondary" className="m-4 select-none">
            <Link href="/login">Get Started</Link>
          </Button>
        </div>
      </div>
    </div>
  )
}
