import Image, { ImageProps } from 'next/image'

export const Icon = (props: Omit<ImageProps, 'src' | 'alt'>) => {
  return (
    <div className="select-none">
      <Image
        src="/icon-white.svg"
        alt="icon"
        className="hidden dark:inline"
        {...props}
      />
      <Image
        src="/icon-black.svg"
        alt="icon"
        className="inline dark:hidden"
        {...props}
      />
    </div>
  )
}
