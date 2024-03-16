// Copyright 2023-2024 Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: Apache-2.0 or MIT

import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

const clx = (...inputs: ClassValue[]) => twMerge(clsx(inputs))

export { clx }
