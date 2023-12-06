import React from 'react'

import { cn } from '@/libraries/utils'

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(({ className, ...props }, ref) => {
  return (
    <button
      className={cn(
        className,
        'rounded-lg border border-gray-300 bg-gray-200 px-5 py-2 text-center text-sm font-medium text-gray-600 transition-all hover:border-gray-200 hover:bg-blue-500 hover:text-white focus:ring focus:ring-gray-50 disabled:border-gray-50 disabled:bg-gray-50 disabled:text-gray-400 dark:bg-gray-100'
      )}
      ref={ref}
      {...props}
    />
  )
})

Button.displayName = 'Button'

export { Button }
