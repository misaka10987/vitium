"use client"
import { Button } from '@/components/ui/button'
import Link from 'next/link'
import { useState } from 'react'

export default function Home() {
  const [isLoggedIn, setIsLoggedIn] = useState(false)
  const [username, setUsername] = useState('')

  // // If logged in, render game view with the username
  // if (isLoggedIn) {
  //   return <GameView username={username} />;
  // }

  // // If not logged in, render login page
  // return (
  //   <PlayerLoginPage 
  //     setIsLoggedIn={setIsLoggedIn}
  //     setUsername={setUsername}
  //   />
  // );
  return <div className='flex h-full justify-center items-center'>
    <div>
      <div className='mb-8'>
        <img src='/icon.svg' width={320} className='pl-4' />
        <h1 className='text-4xl font-bold text-center mb-4'>Vitium</h1>
        <p className='text-center'>The framework for TRPG.</p>
      </div>
      <div>
      {/* <DarkmodeSwitch/>
      <Button>Button</Button>
      <Button variant='destructive'> Click</Button>
        <Button asChild>
          <Link href='/login'>Player Login</Link>
        </Button> */}
      </div>
    </div>
  </div>
}
