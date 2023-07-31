import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export const disableBrowserEvents = (eventName: string) => {
  return document.addEventListener(
    eventName,
    (e) => {
      e.preventDefault()
      return false
    },
    { capture: true }
  )
}
