"use client"

import { BACKEND_URL } from '../../lib/export'


export default function Login() {
  return <div>
    <a
      href={`${BACKEND_URL}/auth/google`}
      className='text-lg border border-stone-700 rounded-2xl p-2.5'
    >Log In</a>
  </div>
}
