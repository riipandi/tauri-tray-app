import useDeviceInfo from '@/hooks/useDeviceInfo'

export function TailwindIndicator({ withScreenSize = false }: { withScreenSize?: boolean }) {
  const { windowSize } = useDeviceInfo()
  const screenSize = withScreenSize ? `: ${windowSize.width}x${windowSize.height}` : ''

  return import.meta.env.DEV ? (
    <div className='fixed bottom-0 right-0 z-50 flex h-6 w-auto items-center justify-center rounded-tl-md bg-gray-800 px-1.5 font-sans text-xs text-white'>
      <div className='block sm:hidden'>xs {screenSize}</div>
      <div className='hidden sm:block md:hidden'>sm {screenSize}</div>
      <div className='hidden md:block lg:hidden'>md {screenSize}</div>
      <div className='hidden lg:block xl:hidden'>lg {screenSize}</div>
      <div className='hidden xl:block 2xl:hidden'>xl {screenSize}</div>
      <div className='hidden 2xl:block'>2xl {screenSize}</div>
    </div>
  ) : null
}
