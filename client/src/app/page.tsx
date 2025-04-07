"use client"

import { Button } from "@/components/ui/button"
import Image from "next/image"
import Link from "next/link"
import { add } from "vitium-api"

export default function Page() {
  console.debug(`WASM initialized: 2+3=${add(2, 3)}`)
  return <div className='flex h-full justify-center items-center'>
    <div className="mb-32">
      <div className='mb-16'>
        <Image src='/icon.svg' alt="Vitium Icon" width={320} height={320} className='pl-4' />
        <h1 className='text-4xl font-bold text-center mb-4'>Vitium</h1>
        <p className='text-center'>The TRPG Framework</p>
      </div>
      <div className="flex justify-center">
        <Button asChild>
          <Link href='/login'>Player Login</Link>
        </Button>
      </div>
    </div>
  </div>
}
