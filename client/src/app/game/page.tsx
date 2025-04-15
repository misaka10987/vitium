import { Chatbox } from '@/components/chatbox'

export default function Page() {
  return (
    <div className="flex flex-col gap-6">
      <div className="w-1/4 ml-2 mt-2">
        <Chatbox />
      </div>
    </div>
  )
}
