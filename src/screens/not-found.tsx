import { Link } from 'wouter'

export default function NotFoundScreen() {
  return (
    <div className='mx-auto flex h-full min-h-screen w-full flex-col'>
      <header className='mb-auto w-full' aria-hidden></header>
      <div className='px-4 py-10 text-center sm:px-6 lg:px-8'>
        <h1 className='block text-7xl font-bold text-gray-800 dark:text-white sm:text-8xl'>404</h1>
        <div className='mt-6 text-lg text-gray-600 dark:text-gray-400 sm:mt-8'>
          <p className='leading-8'>Oops, something went wrong.</p>
          <p className='leading-8'>Sorry, we couldn&rsquo;t find your page.</p>
        </div>
        <div className='mt-8 flex flex-col items-center justify-center'>
          <Link
            to='/'
            className='inline-flex w-full items-center justify-center gap-2 rounded-md border border-transparent px-3 py-2 text-sm font-semibold text-blue-500 ring-offset-white transition-all hover:text-blue-700 focus:outline-none focus:ring-1 focus:ring-blue-500 focus:ring-offset-2 dark:ring-offset-slate-900 sm:w-auto'
          >
            <svg className='h-2.5 w-2.5' width={20} height={20} viewBox='0 0 16 16' fill='none'>
              <path
                d='M11.2792 1.64001L5.63273 7.28646C5.43747 7.48172 5.43747 7.79831 5.63273 7.99357L11.2792 13.64'
                stroke='currentColor'
                strokeWidth={2}
                strokeLinecap='round'
              />
            </svg>
            Back to main page
          </Link>
        </div>
      </div>
      <footer className='mt-auto py-5 text-center'>
        <div className='mx-auto max-w-7xl px-4 sm:px-6 lg:px-8'>
          <p className='text-sm text-gray-500'>
            © All Rights Reserved. {new Date().getFullYear()}
          </p>
        </div>
      </footer>
    </div>
  )
}
