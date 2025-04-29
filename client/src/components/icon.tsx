import Image from 'next/image'

export const Icon = ({ width, height }: { width: number; height: number }) => {
  return (
    <div className="select-none">
      <Image
        src="/icon-white.svg"
        alt="icon"
        width={width}
        height={height}
        className="hidden dark:inline"
      />
      <Image
        src="/icon-black.svg"
        alt="icon"
        width={width}
        height={height}
        className="inline dark:hidden"
      />
    </div>
  )
}
