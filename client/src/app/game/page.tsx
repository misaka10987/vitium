import { Chat } from '@/components/chat'

export default function Page() {
  return (
    <div className="flex flex-row gap-6 w-full h-full p-2">
      <div className="flex-1/4 max-w-1/4">
        <Chat />
      </div>
    </div>
  )
}
