"use client"
import { useState } from 'react';
import { add } from 'vitium-api';
import { GameView } from './gameview';
import { PlayerLoginPage } from './playerlogin';

export default function Home() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [username, setUsername] = useState('');

  // If logged in, render game view with the username
  if (isLoggedIn) {
    return <GameView username={username} />;
  }
  
  // If not logged in, render login page
  return (
    <PlayerLoginPage 
      setIsLoggedIn={setIsLoggedIn}
      setUsername={setUsername}
    />
  );
}
