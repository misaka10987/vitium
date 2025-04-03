export function GameView({ username }: { username: string }) {
  return (
    <div className="game-container">
      <div className="welcome-banner">
        <h2>Welcome, {username}!</h2>
      </div>
    </div>
  );
}
