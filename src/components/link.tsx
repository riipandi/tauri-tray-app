import { A, type AnchorProps, useLocation } from '@solidjs/router'

export default function Link(props: AnchorProps) {
  const { pathname } = useLocation()
  return <A state={{ previous: pathname }} {...props} />
}
