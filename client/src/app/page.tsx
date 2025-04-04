"use client"

import Image from "next/image"
import { add } from "vitium-api"

export default function Home() {
  console.log(add(2, 3))
  return <div className='flex h-full justify-center items-center'>
    <div>
      <div className='mb-8'>
        <Image src='/icon.svg' alt="Vitium Icon" width={320} className='pl-4' />
        <h1 className='text-4xl font-bold text-center mb-4'>Vitium</h1>
        <p className='text-center'>The framework for TRPG.</p>
      </div>
      <div>
      </div>
    </div>
  </div>
}
