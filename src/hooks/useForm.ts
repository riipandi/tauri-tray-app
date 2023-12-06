/**
 * Originally taken from: https://github.com/jondot/tauri-tray-app/blob/master/src/hooks/use-form.tsx
 */

import type { FieldValues, UseFormProps, UseFormReturn } from 'react-hook-form'
import { useForm as useFormHook } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import type { z, ZodType } from 'zod'

export const useForm = <
  T extends ZodType<any, any, any>,
  TFieldValues extends FieldValues = z.infer<typeof schema>,
  TContext = any,
>(
  props: UseFormProps<TFieldValues, TContext> & { schema: T }
): UseFormReturn<TFieldValues, TContext> => {
  const { schema, resolver: _resolver, ...rest } = props
  return useFormHook<TFieldValues, TContext>({
    resolver: zodResolver(schema),
    ...rest,
  })
}
